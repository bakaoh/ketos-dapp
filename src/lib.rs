//! A template application for Fluence.
use fluence::sdk::*;

fn init() {
    logger::WasmLogger::init_with_level(log::Level::Info).unwrap();
}

#[invocation_handler(init_fn = init)]
fn main(arg: String) -> String {
    return arg
}
