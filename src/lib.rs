extern crate ketos;

use fluence::sdk::*;
use ketos::Interpreter;

fn init() {
    logger::WasmLogger::init_with_level(log::Level::Info).unwrap();
}

#[invocation_handler(init_fn = init)]
fn main(arg: String) -> String {
    let interp = Interpreter::new();
    let compile_rs = interp.compile_exprs(arg.as_str());
    let mut rs = String::new();

    match compile_rs {
        Ok(code) => {
            for c in code {
                match interp.execute(c) {
                    Ok(v) => rs.push_str(interp.format_value(&v).as_str()),
                    Err(e) => rs.push_str(interp.format_error(&e).as_str()),
                }
                rs.push('\n');
            }
        },
        Err(e) => rs.push_str(interp.format_error(&e).as_str()),
    }
    
    return rs;
}
