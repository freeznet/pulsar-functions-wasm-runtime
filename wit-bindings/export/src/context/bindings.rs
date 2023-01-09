#[allow(clippy::all)]
pub mod context {
    #[allow(unused_imports)]
    use wit_bindgen_host_wasmtime_rust::{anyhow, wasmtime};
    #[derive(Clone)]
    pub struct PulsarRecord {
        pub value: Vec<u8>,
        pub key: String,
    }
    impl core::fmt::Debug for PulsarRecord {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.debug_struct("PulsarRecord")
                .field("value", &self.value)
                .field("key", &self.key)
                .finish()
        }
    }
    pub trait Context: Sized {
        fn get_current_message(&mut self) -> PulsarRecord;

        fn set_output_message(&mut self, data: &[u8]) -> ();
    }

    pub fn add_to_linker<T, U>(
        linker: &mut wasmtime::Linker<T>,
        get: impl Fn(&mut T) -> &mut U + Send + Sync + Copy + 'static,
    ) -> anyhow::Result<()>
    where
        U: Context,
    {
        use wit_bindgen_host_wasmtime_rust::rt::get_func;
        use wit_bindgen_host_wasmtime_rust::rt::get_memory;
        linker.func_wrap(
            "context",
            "get-current-message",
            move |mut caller: wasmtime::Caller<'_, T>, arg0: i32| {
                let func = get_func(&mut caller, "cabi_realloc")?;
                let func_cabi_realloc = func.typed::<(i32, i32, i32, i32), i32, _>(&caller)?;
                let memory = &get_memory(&mut caller, "memory")?;
                let host = get(caller.data_mut());
                let result0 = host.get_current_message();
                let PulsarRecord {
                    value: value1,
                    key: key1,
                } = result0;
                let vec2 = value1;
                let ptr2 =
                    func_cabi_realloc.call(&mut caller, (0, 0, 1, (vec2.len() as i32) * 1))?;
                let caller_memory = memory.data_mut(&mut caller);
                caller_memory.store_many(ptr2, &vec2)?;
                caller_memory.store(
                    arg0 + 4,
                    wit_bindgen_host_wasmtime_rust::rt::as_i32(vec2.len() as i32),
                )?;
                caller_memory.store(arg0 + 0, wit_bindgen_host_wasmtime_rust::rt::as_i32(ptr2))?;
                let vec3 = key1;
                let ptr3 = func_cabi_realloc.call(&mut caller, (0, 0, 1, vec3.len() as i32))?;
                let caller_memory = memory.data_mut(&mut caller);
                caller_memory.store_many(ptr3, vec3.as_bytes())?;
                caller_memory.store(
                    arg0 + 12,
                    wit_bindgen_host_wasmtime_rust::rt::as_i32(vec3.len() as i32),
                )?;
                caller_memory.store(arg0 + 8, wit_bindgen_host_wasmtime_rust::rt::as_i32(ptr3))?;
                Ok(())
            },
        )?;
        linker.func_wrap(
            "context",
            "set-output-message",
            move |mut caller: wasmtime::Caller<'_, T>, arg0: i32, arg1: i32| {
                let memory = &get_memory(&mut caller, "memory")?;
                let (mem, data) = memory.data_and_store_mut(&mut caller);
                let mut _bc = wit_bindgen_host_wasmtime_rust::BorrowChecker::new(mem);
                let host = get(data);
                let ptr0 = arg0;
                let len0 = arg1;
                let param0 = _bc.slice(ptr0, len0)?;
                host.set_output_message(param0);
                Ok(())
            },
        )?;
        Ok(())
    }
    use wit_bindgen_host_wasmtime_rust::rt::RawMem;
}
