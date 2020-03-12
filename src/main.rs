include!("lib.rs");

fn main() {
    let mut env = environment::new_environment();
    let mut l = lexer::Lexer::new(
        //         "
        // let newAdder = fn(x) {
        //     fn(y) { x + y };
        // };
        // let addTwo = newAdder(2);
        // addTwo(2);
        //    ",
        "let add = fn(x, y) { x + y; }; add(5 + 5, add(5, 5));",
    );
    let mut p = parser::Parser::new(&mut l);
    let program = p.parse_program().unwrap();
    let result = evaluator::eval(&program, &mut env);
    println!("{:#?}", result);
    println!("Hello! This is the Monkey programming language!");
    println!("Feel free to type in commands");
    repl::start(&mut std::io::stdin(), &mut std::io::stdout());
}
