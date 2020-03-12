extern crate waiir;
use waiir::ast::*;
use waiir::environment::*;
use waiir::evaluator::*;
use waiir::lexer::*;
use waiir::object::*;
use waiir::parser::*;

fn test_eval(input: &str) -> Box<dyn Object> {
    let mut env = new_environment();
    let mut l = Lexer::new(input);
    let mut p = Parser::new(&mut l);
    let program = p.parse_program().unwrap();

    eval(&program, &mut env).unwrap()
}

#[test]
fn test_eval_integer_expression() {
    let tests = [
        ("5", 5),
        ("10", 10),
        ("-5", -5),
        ("-10", -10),
        ("5 + 5 + 5 + 5 - 10", 10),
        ("2 * 2 * 2 * 2 * 2", 32),
        ("-50 + 100 + -50", 0),
        ("5 * 2 + 10", 20),
        ("5 + 2 * 10", 25),
        ("20 + 2 * -10", 0),
        ("50 / 2 * 2 + 10", 60),
        ("2 * (5 + 10)", 30),
        ("3 * 3 * 3 + 10", 37),
        ("3 * (3 * 3) + 10", 37),
        ("(5 + 10 * 2 + 15 / 3) * 2 + -10", 50),
    ];
    for tt in tests.iter() {
        let evaluated = test_eval(tt.0);
        test_integer_object(evaluated, tt.1);
    }
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

#[test]
fn test_eval_boolean_expression() {
    let tests = [
        ("true", true),
        ("false", false),
        ("1 < 2", true),
        ("1 > 2", false),
        ("1 < 1", false),
        ("1 > 1", false),
        ("1 == 1", true),
        ("1 != 1", false),
        ("1 == 2", false),
        ("1 != 2", true),
        ("true == false", false),
        ("true != false", true),
        ("false != true", true),
        ("(1 < 2) == true", true),
        ("(1 < 2) == false", false),
        ("(1 > 2) == true", false),
        ("(1 > 2) == false", true),
    ];
    for tt in tests.iter() {
        let evaluated = test_eval(tt.0);
        test_boolean_object(evaluated, tt.1);
    }
}

fn test_boolean_object(obj: Box<dyn Object>, expected: bool) {
    let result = obj
        .as_any()
        .downcast_ref::<waiir::object::Boolean>()
        .expect(&format!("object is not Boolean. got={:?}", obj));
    assert!(
        result.value == expected,
        "object has wrong value. got={}, want={}",
        result.value,
        expected
    );
}

#[test]
fn test_bang_operator() {
    let tests = [
        ("!true", false),
        ("!false", true),
        ("!5", false),
        ("!!true", true),
        ("!!false", false),
        ("!!5", true),
    ];
    for tt in tests.iter() {
        let evaluated = test_eval(tt.0);
        test_boolean_object(evaluated, tt.1);
    }
}

#[test]
fn test_if_else_expressions() {
    let tests: [(&str, Box<dyn Object>); 7] = [
        ("if (true) { 10 }", Box::new(Integer { value: 10 })),
        ("if (false) { 10 }", Box::new(Null {})),
        ("if (1) { 10 }", Box::new(Integer { value: 10 })),
        ("if (1 < 2) { 10 }", Box::new(Integer { value: 10 })),
        ("if (1 > 2) { 10 }", Box::new(Null {})),
        (
            "if (1 > 2) { 10 } else { 20 }",
            Box::new(Integer { value: 20 }),
        ),
        (
            "if (1 < 2) { 10 } else { 20 }",
            Box::new(Integer { value: 10 }),
        ),
    ];

    for tt in tests.iter() {
        let evaluated = test_eval(tt.0);
        match tt.1.as_any().downcast_ref::<Integer>() {
            Some(integer) => test_integer_object(evaluated, integer.value),
            _ => test_null_object(evaluated),
        }
    }
}

fn test_null_object(obj: Box<dyn Object>) {
    obj.as_any()
        .downcast_ref::<Null>()
        .expect(&format!("object is not NULL. got={:?}", obj));
}

#[test]
fn test_return_statement() {
    let tests = [
        ("return 10;", 10),
        ("return 10; 9;", 10),
        ("return 2 * 5; 9;", 10),
        ("9; return 2 * 5; 9;", 10),
        (
            "if (10 > 1) { if (10 > 1) {
            return 10; }
            return 1; }",
            10,
        ),
    ];

    for tt in tests.iter() {
        let evaluated = test_eval(tt.0);
        test_integer_object(evaluated, tt.1);
    }
}

#[test]
fn test_error_handling() {
    let tests = [
        ("5 + true;", "type mismatch: INTEGER + BOOLEAN"),
        ("5 + true; 5;", "type mismatch: INTEGER + BOOLEAN"),
        ("-true", "unknown operator: -BOOLEAN"),
        ("true + false;", "unknown operator: BOOLEAN + BOOLEAN"),
        ("5; true + false; 5", "unknown operator: BOOLEAN + BOOLEAN"),
        (
            "if (10 > 1) { true + false; }",
            "unknown operator: BOOLEAN + BOOLEAN",
        ),
        (
            "if (10 > 1) {
                if (10 > 1) {
                    return true + false;
                }
                return 1;
            }",
            "unknown operator: BOOLEAN + BOOLEAN",
        ),
        ("foobar", "identifier not found: foobar"),
    ];

    for tt in tests.iter() {
        let evaluated = test_eval(tt.0);
        let err_obj = evaluated
            .as_any()
            .downcast_ref::<Error>()
            .expect(&format!("no error object returned. got={:?}", evaluated));
        assert!(
            err_obj.message == tt.1,
            "wrong error message. expected={}, got={}",
            err_obj.message,
            tt.1
        );
    }
}

#[test]
fn test_let_statements() {
    let tests = [
        ("let a = 5; a;", 5),
        ("let a = 5 * 5; a;", 25),
        ("let a = 5; let b = a; b;", 5),
        ("let a = 5; let b = a; let c = a + b + 5; c;", 15),
    ];

    for tt in tests.iter() {
        test_integer_object(test_eval(tt.0), tt.1);
    }
}

#[test]
fn test_function_object() {
    let input = "fn(x) { x + 2; };";
    let evaluated = test_eval(input);
    let func = evaluated
        .as_any()
        .downcast_ref::<FUNCTION>()
        .expect(&format!("object is not FUNCTION. got={:?}", evaluated));

    assert!(
        func.function_literal.parameters.len() == 1,
        "function has wrong parameters. parameters={:?}",
        func.function_literal.parameters
    );

    assert!(
        func.function_literal.parameters[0].string() == "x",
        "parameter is not 'x'. got={:?}",
        func.function_literal.parameters[0]
    );

    let expected_body = "(x + 2)";
    assert!(
        func.function_literal.body.string() == expected_body,
        "body is not {}. got={}",
        expected_body,
        func.function_literal.body.string()
    );
}

#[test]
fn test_function_application() {
    let tests = [
        ("let identity = fn(x) { x; }; identity(5);", 5),
        ("let identity = fn(x) { return x; }; identity(5);", 5),
        ("let double = fn(x) { x * 2; }; double(5);", 10),
        ("let add = fn(x, y) { x + y; }; add(5, 5);", 10),
        ("let add = fn(x, y) { x + y; }; add(5 + 5, add(5, 5));", 20),
        ("fn(x) { x; }(5)", 5),
    ];

    for tt in tests.iter() {
        test_integer_object(test_eval(tt.0), tt.1);
    }
}

#[test]
fn test_closures() {
    let input = "let newAdder = fn(x) {
            fn(y) { x + y };
        };
        let addTwo = newAdder(2);
        addTwo(2);
        ";

    test_integer_object(test_eval(input), 4);
}
