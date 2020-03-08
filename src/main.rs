include!("lib.rs");

use crate::ast::*;

fn main() {
    let mut l = lexer::Lexer::new("-a * b");
    let mut p = parser::Parser::new(&mut l);
    let program = p.parse_program().unwrap();
    println!("{}", program.string());
    println!("Hello! This is the Monkey programming language!");
    println!("Feel free to type in commands");
    repl::start(&mut std::io::stdin(), &mut std::io::stdout());
}
