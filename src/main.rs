mod ast;
mod lexer;
mod parser;
mod repl;

fn main() {
    let mut l = lexer::Lexer::new("!5;");
    let mut p = parser::Parser::new(&mut l);
    p.parse_program();
    println!("Hello! This is the Monkey programming language!");
    println!("Feel free to type in commands");
    repl::start(&mut std::io::stdin(), &mut std::io::stdout());
}
