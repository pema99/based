pub mod ast;
pub mod parser;
#[allow(unused_must_use)]
pub mod scanner;

fn main() {
    let src = std::fs::read_to_string("./program.c").unwrap();

    let scanner = scanner::Scanner::new(src.chars());
    let tokens = scanner.scan_all().unwrap();
    let program = parser::parse_program(&mut tokens.into_iter().peekable());

    println!("{:?}", program);
}
