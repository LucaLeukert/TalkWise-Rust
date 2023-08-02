#![allow(dead_code)]

use crate::compiler::Compiler;

mod compiler;
mod lexer;
mod error;
mod parser;

fn main() -> Result<(), error::TalkError> {
    println!("TalkWise compiler written in Rust.");

    let mut compiler = Compiler::new();
    let file_result = compiler.add_file("test.talk".as_ref());

    if file_result.is_err() {
        return Err(file_result.unwrap_err());
    }

    let compile_result = compiler.compile();

    if compile_result.is_err() {
        return Err(compile_result.unwrap_err());
    }

    Ok(())
}
