#[macro_use]
extern crate serde;
use std::borrow::Borrow;
use std::env;
use std::ops::Deref;
use std::sync::{Arc, Mutex};
use std::str;

use anyhow::Result;
use futures::TryStreamExt;
use wasi_common::WasiCtx;
use wasmtime::*;
use wasmtime_wasi::sync::WasiCtxBuilder;
use pulsar::{
    message::{proto, proto::command_subscribe::SubType, Payload},
    producer, consumer::ConsumerOptions, proto::Schema, reader::Reader, DeserializeMessage, Error as PulsarError, Pulsar, SerializeMessage,
    TokioExecutor,
};
use pfwasm_wit_bindings::context::context::*;

struct HostStateContext {
    pub wasi: WasiCtx,
    pub func_context: FunctionContext,
}

pub struct FunctionContext {
    pub current_record: PulsarRecord,
    pub output_message: Option<Vec<u8>>,
}

impl Context for FunctionContext {
    fn get_current_message(&mut self) -> PulsarRecord {
        self.current_record.clone()
    }

    fn set_output_message(&mut self, data: &[u8]) -> () {
        self.output_message = Some(data.to_vec());
    }
}

#[tokio::main]
async fn main() -> Result<()> {

    let addr = env::var("PULSAR_URL")
        .ok()
        .unwrap_or_else(|| "pulsar://127.0.0.1:6650".to_string());
    let output = env::var("PULSAR_OUTPUT_TOPIC")
        .ok()
        .unwrap_or_else(|| "persistent://public/default/output".to_string());
    let input = env::var("PULSAR_INPUT_TOPIC")
        .ok()
        .unwrap_or_else(|| "persistent://public/default/input".to_string());

    let mut builder = Pulsar::builder(addr, TokioExecutor);

    let pulsar: Pulsar<_> = builder.build().await?;
    println!("connected to pulsar");

    let mut producer = pulsar
        .producer()
        .with_topic(output)
        .with_name("my producer")
        .with_options(producer::ProducerOptions {
            schema: Some(proto::Schema {
                r#type: proto::schema::Type::String as i32,
                ..Default::default()
            }),
            ..Default::default()
        })
        .build()
        .await?;

    let mut reader: Reader<String, _> = pulsar
        .reader()
        .with_topic(input)
        .with_consumer_name("test_reader")
        .with_options(ConsumerOptions::default().with_schema(Schema {
            r#type: pulsar::proto::schema::Type::String as i32,
            ..Default::default()
        }))
        // subscription defaults to SubType::Exclusive
        .into_reader()
        .await?;


    let engine = Engine::default();
    println!("engine created");
    let module = Module::from_file(&engine, "exclamation.wasm")?;
    println!("module created, load exclamation.wasm");
    // listen to 5 messages
    while let Some(msg) = reader.try_next().await? {
        println!("metadata: {:#?}", msg.metadata());

        println!("id: {:?}", msg.message_id());
        
        let mut linker: Linker<HostStateContext> = Linker::new(&engine);

        let wasi = WasiCtxBuilder::new()
            .inherit_stdout()
            .inherit_stderr()
            .build();

        let func_context: FunctionContext = FunctionContext {
            current_record: PulsarRecord {
                value: msg.payload.data.clone(),
                key: msg.key().unwrap_or_default(),
            },
            output_message: None,
        };
        
        let ctx: HostStateContext = HostStateContext {
            wasi,
            func_context,
        };

        let mut store = Store::new(&engine, ctx);
        wasmtime_wasi::add_to_linker(&mut linker, |s: &mut HostStateContext | &mut s.wasi)?;
        println!("linked module");

        add_to_linker(&mut linker, |ctx: &mut HostStateContext| -> &mut FunctionContext {
            &mut ctx.func_context
        })?;

        linker
            .module(&mut store, "", &module)
            .expect("linking the function");
        linker
            .get_default(&mut store, "")
            .expect("should get the wasi runtime")
            .typed::<(), (), _>(&store)
            .expect("should type the function")
            .call(&mut store, ())
            .expect("should call the function");
        
        let output = &store.data().func_context.output_message;
        println!("output: {:?}", output);
        match output {
            Some(v) => Some(producer.send(v.clone()).await?.await.unwrap()),
            None => None,
        };

        drop(store);
    }

    Ok(())
}