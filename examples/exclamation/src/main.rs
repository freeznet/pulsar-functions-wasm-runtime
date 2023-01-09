use std::str;
use std::error::Error;
use pfwasm_bindings::{context as function_context};


fn main() -> Result<(), Box<dyn Error>> {
    let record = function_context::get_current_message();

    println!("record = {:?}", record);

    let input = match str::from_utf8(&record.value) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };

    function_context::set_output_message(format!("Hello, {}!", input).as_bytes());

    Ok(())
}
