#[macro_use]
extern crate serde;
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

struct HostStateContext {
    pub wasi: WasiCtx,
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

    let mut counter = 0usize;
    let engine = Engine::default();
    let module = Module::from_file(&engine, "exclamation.wasm")?;
    // listen to 5 messages
    while let Some(msg) = reader.try_next().await? {
        println!("metadata: {:#?}", msg.metadata());

        println!("id: {:?}", msg.message_id());
        
        let mut linker: Linker<HostStateContext> = Linker::new(&engine);

        let wasi = WasiCtxBuilder::new()
            .inherit_stdout()
            .inherit_stderr()
            .build();
        
        let ctx: HostStateContext = HostStateContext {
            wasi,
        };

        let mut store = Store::new(&engine, ctx);
        wasmtime_wasi::add_to_linker(&mut linker, |s| &mut s.wasi)?;
        println!("linked module");
        let memory_ty = MemoryType::new(1, None);
        Memory::new(&mut store, memory_ty).expect("memory");

        let buf = msg.payload.data.clone();
        let mem_size: i32 = buf.len() as i32;

        linker
            .func_wrap("host", "get_input_size", move || -> i32 { mem_size })
            .expect("should define the function");

        linker
            .func_wrap(
                "host",
                "get_input",
                move |mut caller: Caller<'_, HostStateContext>, ptr: i32| {
                    let mem = match caller.get_export("memory") {
                        Some(Extern::Memory(mem)) => mem,
                        _ => return Err(anyhow::Error::msg("failed to find host memory")),
                    };
                    let offset = ptr as u32 as usize;
                    match mem.write(&mut caller, offset, &buf) {
                        Ok(_) => {}
                        _ => return Err(anyhow::Error::msg("failed to write to host memory")),
                    };
                    Ok(())
                },
            )
            .expect("should define the function");

        let output: Arc<Mutex<Vec<u8>>> = Arc::new(Mutex::new(Vec::new()));

        let output_ = output.clone();

        linker
            .func_wrap(
                "host",
                "set_output",
                move |mut caller: Caller<'_, HostStateContext>, ptr: i32, capacity: i32| {
                    let output = output_.clone();
                    let mem = match caller.get_export("memory") {
                        Some(Extern::Memory(mem)) => mem,
                        _ => return Err(anyhow::Error::msg("failed to find host memory")),
                    };
                    let offset = ptr as u32 as usize;
                    let mut buffer: Vec<u8> = vec![0; capacity as usize];
                    match mem.read(&caller, offset, &mut buffer) {
                        Ok(()) => {
                            println!(
                                "Buffer = {:?}, ptr = {}, capacity = {}",
                                buffer, ptr, capacity
                            );
                            let mut output = output.lock().unwrap();
                            *output = buffer;
                            Ok(())
                        }
                        _ => Err(anyhow::Error::msg("failed to read host memory")),
                    }
                },
            )
            .expect("should define the function");

        linker
            .module(&mut store, "", &module)
            .expect("linking the function");
        linker
            .get_default(&mut store, "")
            .expect("should get the wasi runtime")
            .typed::<(), ()>(&store)
            .expect("should type the function")
            .call(&mut store, ())
            .expect("should call the function");
        
        drop(store);

        let output = output.lock();
        println!("output: {:?}", output);

        if let Ok(data) = output {
            match str::from_utf8(data.deref()) {
                Ok(v) => producer.send(v).await?.await.unwrap(),
                Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
            };
        }
    }

    Ok(())
}