#![allow(dead_code)]
#![forbid(unsafe_code)]

mod parser;
mod rpn;

fn main() {
    if let Err(err) = parser::rpn_repl() {
        println!("Error: {:?}", err);
    }
}
