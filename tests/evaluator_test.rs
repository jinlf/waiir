extern crate waiir;
use waiir::evaluator::*;
use waiir::lexer::*;
use waiir::object::*;
use waiir::parser::*;

#[test]
fn test_eval_integer_expression() {
    let tests = [("5", 5), ("10", 10)];
    for tt in tests.iter() {
        let evaluated = test_eval(tt.0).unwrap();
        test_integer_object(evaluated, tt.1);
    }
}

fn test_eval(input: &str) -> Option<Box<dyn Object>> {
    let mut l = Lexer::new(input);
    let mut p = Parser::new(&mut l);
    let program = p.parse_program().unwrap();

    eval(&program)
}

fn test_integer_object(obj: Box<dyn Object>, expected: i64) {
    let result = obj
        .as_any()
        .downcast_ref::<Integer>()
        .expect(&format!("object is not Integer. got={:?}", obj));

    assert!(
        result.value == expected,
        "object has wrong value. got={}, want={}",
        result.value,
        expected
    );
}
