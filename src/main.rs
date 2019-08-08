extern crate ketos;

use ketos::{Error, Interpreter};

fn main() {
    // First, create an interpreter.
    let interp = Interpreter::new();
    let code = interp
        .compile_exprs(
            r#"
            1
        (define (factorial n)
          (cond
            ((< n 0) (panic "factorial got negative integer"))
            ((<= n 1) 1)
            (else (* n (factorial (- n 1))))))
        (factorial 5)
        (factorial 6)

        (factorial )
        "#,
        )
        .unwrap();

    let mut rs = String::new();
    for c in code {
        match interp.execute(c) {
            Ok(v) => rs.push_str(interp.format_value(&v).as_str()),
            Err(e) => rs.push_str(interp.format_error(&e).as_str()),
        }
        rs.push('\n');
    }

    println!("{}", rs.as_str());
}
