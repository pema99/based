#![allow(unused_must_use)]
#![allow(unused_variables)]

pub mod ast;
pub mod parser;
pub mod scanner;
pub mod compile;
pub mod asm;

fn main() {
    let src = std::fs::read_to_string("./program.c").unwrap();

    let scanner = scanner::Scanner::new(src.chars());
    let tokens = scanner.scan_all().unwrap();
    let program = parser::parse_program(&mut tokens.into_iter().peekable());
    let compiler = compile::Compiler::new();
    let blob = compiler.compile_program(&program.unwrap());

    asm::print_asm(blob);
}
