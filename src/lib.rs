extern crate ketos;

use fluence::sdk::*;
use ketos::Interpreter;

fn init() {
    logger::WasmLogger::init_with_level(log::Level::Info).unwrap();
}

#[invocation_handler(init_fn = init)]
fn main(arg: String) -> String {
    let interp = Interpreter::new();
    let code = interp.compile_exprs(arg.as_str()).unwrap();
    let mut rs = String::new();

    for c in code {
        match interp.execute(c) {
            Ok(v) => rs.push_str(interp.format_value(&v).as_str()),
            Err(e) => rs.push_str(interp.format_error(&e).as_str()),
        }
        rs.push('\n');
    }
    return rs;
}
