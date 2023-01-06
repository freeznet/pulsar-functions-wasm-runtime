use std::str;
use std::error::Error;

#[link(wasm_import_module = "host")]
extern "C" {
    fn get_input_size() -> i32;
    fn get_input(ptr: i32);
    fn set_output(ptr: i32, size: i32);
}


fn main() -> Result<(), Box<dyn Error>> {
    let mem_size = unsafe { get_input_size() };

    let mut buf: Vec<u8> = Vec::with_capacity(mem_size as usize);
    let ptr = buf.as_mut_ptr();
    std::mem::forget(ptr);

    let input_buf = unsafe {
        get_input(ptr as i32);
        Vec::from_raw_parts(ptr, mem_size as usize, mem_size as usize)
    };

    println!("input_buf = {:?}", input_buf);

    let input = match str::from_utf8(&input_buf) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };

    println!("input = {:?}", input);

    let output = format!("{}!", input);
    let serialized = output.as_bytes().to_vec();
    let size = serialized.len() as i32;
    let ptr = serialized.as_ptr();
    std::mem::forget(ptr);

    unsafe {
        set_output(ptr as i32, size);
    }

    Ok(())
}