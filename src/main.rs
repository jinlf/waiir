mod lexer;
mod repl;
mod parser;
mod ast;

fn main() {
    // println!("Hello! This is the Monkey programming language!");
    // println!("Feel free to type in commands");
    // repl::start(&mut std::io::stdin(), &mut std::io::stdout());

    let l = lexer::Lexer::new("!5;");
    let mut p = parser::Parser::new(l);
    let program = p.parse_program();
    println!("hello");
}
