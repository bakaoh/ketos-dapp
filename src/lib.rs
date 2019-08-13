extern crate ketos;

use fluence::sdk::*;
use ketos::Interpreter;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::collections::HashMap;

thread_local!{
    static SESSION: RefCell<HashMap<String, Interpreter>> = RefCell::new(HashMap::new());
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "action")]
pub enum Request {
    Run { session: String, code: String },
}

fn init() {
    logger::WasmLogger::init_with_level(log::Level::Info).unwrap();
}

fn run(session: String, arg: String) -> Result<String, String> {
    SESSION.with(|s| {
        let mut session_map = s.borrow_mut();
        Ok(match session_map.get(&session) {
            Some(interp) => run_with_interp(interp, arg),
            None => {
                let interp = Interpreter::new();
                let rs = run_with_interp(&interp, arg);
                session_map.insert(session.clone(), interp);
                rs
            }
        })
    })
}

fn run_with_interp(interp: &Interpreter, arg: String) -> String {
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

#[invocation_handler(init_fn = init)]
fn main(arg: String) -> String {
    match serde_json::from_str(arg.as_str()) {
        Ok(req) => {
            let res = match req {
                Request::Run { session, code } => {
                    run(session, code)
                }
            };
            match res {
                Ok(rs) => rs,
                Err(e) => e,
            }
        }
        Err(_) => String::from("Invalid request"),
    }
}
