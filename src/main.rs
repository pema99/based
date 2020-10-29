#![allow(unused_must_use)]
#![allow(unused_variables)]

pub mod ast;
pub mod parser;
pub mod scanner;
pub mod compile;

fn main() {
    let src = std::fs::read_to_string("./program.c").unwrap();

    let scanner = scanner::Scanner::new(src.chars());
    let tokens = scanner.scan_all().unwrap();
    let program = parser::parse_program(&mut tokens.into_iter().peekable());
    let mut compiler = compile::Compiler::new();
    compiler.compile_program(&program.unwrap());

    println!("{:#?}", compiler);
}
