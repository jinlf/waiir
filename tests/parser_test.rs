extern crate waiir;

use std::any::Any;
use waiir::ast::*;
use waiir::lexer::*;
use waiir::parser::*;

fn check_parser_errors(p: &Parser) {
    let errors = &p.get_errors();
    if errors.len() == 0 {
        return;
    }
    println!("parser has {} errors", errors.len());
    for msg in errors.iter() {
        println!("parser error: {}", *msg);
    }
    assert!(false, "parser error!!!");
}

#[test]
fn test_let_statements() {
    let tests: [(&str, &str, Box<dyn Any>); 3] = [
        ("let x = 5;", "x", Box::new(5 as i64)),
        ("let y = true;", "y", Box::new(true)),
        ("let foobar = y;", "foobar", Box::new("y")),
    ];
    for tt in tests.iter() {
        let mut l = Lexer::new(tt.0);
        let mut p = Parser::new(&mut l);
        let program = p.parse_program().expect("parse_program() returned None");
        check_parser_errors(&p);
        assert!(
            program.statements.len() == 1,
            "program.statements does not contain 1 statements. got={}",
            program.statements.len()
        );
        let stmt = &program.statements[0];
        test_let_statement(stmt, tt.1);

        let val = &stmt.as_any().downcast_ref::<LetStatement>().unwrap().value;
        test_literal_expression(val, &tt.2);
    }
}

fn test_let_statement(s: &Box<dyn Statement>, name: &str) {
    assert!(
        s.token_literal() == "let",
        "s.token_literal not 'let'. got={}",
        s.token_literal()
    );
    let let_stmt = s
        .as_any()
        .downcast_ref::<LetStatement>()
        .expect(&format!("s not ast.LetStatement. got={:?}", s));

    assert!(
        let_stmt.name.value == *name,
        "let_stmt.name.value not '{}'. got={}",
        name,
        let_stmt.name.value
    );
    assert!(
        let_stmt.name.token_literal() == name,
        "s.name not '{}'. got={}",
        name,
        let_stmt.name.token_literal()
    );
}

#[test]
fn test_return_statements() {
    let input = "
        return 5;
        return 10;
        return 993322;
        ";
    let mut l = Lexer::new(&input);
    let mut p = Parser::new(&mut l);

    let program = p.parse_program().expect("parse_program() returned None");

    check_parser_errors(&p);
    assert!(
        program.statements.len() == 3,
        "program.statements does not contain 3 statements. got={}",
        program.statements.len()
    );
    for stmt in program.statements.iter() {
        let return_stmt = stmt
            .as_any()
            .downcast_ref::<ReturnStatement>()
            .expect(&format!("stmt not ReturnStatment. got={:?}", stmt));
        assert!(
            stmt.token_literal() == "return",
            "return_stmt.token_literal not 'return', got {}",
            return_stmt.token_literal()
        );
    }
}

#[test]
fn test_string() {
    let program = Program {
        statements: vec![Box::new(LetStatement {
            token: Token {
                tk_type: TokenType::LET,
                literal: String::from("let"),
            },
            name: Identifier {
                token: Token {
                    tk_type: TokenType::IDENT,
                    literal: String::from("myVar"),
                },
                value: String::from("myVar"),
            },
            value: Box::new(Identifier {
                token: Token {
                    tk_type: TokenType::IDENT,
                    literal: String::from("anotherVar"),
                },
                value: String::from("anotherVar"),
            }),
        }) as Box<dyn Statement>],
    };

    assert!(
        program.string() == "let myVar = anotherVar;",
        "program.string() wrong. got={}",
        program.string()
    );
}

#[test]
fn test_identifier_expresion() {
    let input = "foobar;";
    let mut l = Lexer::new(input);
    let mut p = Parser::new(&mut l);
    let program = p.parse_program().expect("parse_program() returned nil");
    check_parser_errors(&mut p);
    assert!(
        program.statements.len() == 1,
        "program has not enough statements. got={}",
        program.statements.len()
    );
    let stmt = &program.statements[0]
        .as_any()
        .downcast_ref::<ExpressionStmt>()
        .expect(&format!(
            "program.statements[0] is not ExpressionStmt. got={:?}",
            program.statements[0]
        ));
    let ident = stmt
        .expression
        .as_any()
        .downcast_ref::<Identifier>()
        .expect(&format!("exp not Identifier. got={:?}", stmt.expression));
    assert!(
        ident.value == "foobar",
        "ident.value not {}. got={}",
        "foobar",
        ident.value
    );
    assert!(
        ident.token_literal() == "foobar",
        "ident.token_literal not {}. got={}",
        "foobar",
        ident.token_literal()
    );
}

#[test]
fn test_integer_literal_expression() {
    let input = "5;";
    let mut l = Lexer::new(input);
    let mut p = Parser::new(&mut l);
    let program = p.parse_program().expect("parse_program() returned nil");
    check_parser_errors(&mut p);
    assert!(
        program.statements.len() == 1,
        "program has not enough statements. got={}",
        program.statements.len()
    );
    let stmt = program.statements[0]
        .as_any()
        .downcast_ref::<ExpressionStmt>()
        .expect(&format!(
            "program.statements[0] is not ast.ExpressionStmt. got={:?}",
            program.statements[0]
        ));

    test_integer_literal(&stmt.expression, 5);
}

#[test]
fn test_parsing_prefix_expressions() {
    let prefix_tests: [(&str, &str, Box<dyn Any>); 4] = [
        ("!5;", "!", Box::new(5 as i64)),
        ("-15;", "-", Box::new(15 as i64)),
        ("!true", "!", Box::new(true)),
        ("!false", "!", Box::new(false)),
    ];
    for tt in prefix_tests.iter() {
        let mut l = Lexer::new(tt.0);
        let mut p = Parser::new(&mut l);
        let program = p.parse_program().expect("parse_program() returned None");
        check_parser_errors(&mut p);
        assert!(
            program.statements.len() == 1,
            "program has not enough statements. got={}",
            program.statements.len()
        );
        let stmt = program.statements[0]
            .as_any()
            .downcast_ref::<ExpressionStmt>()
            .expect(&format!(
                "program.statements[0] is not ast.ExpressionStmt. got={:?}",
                program.statements[0]
            ));
        let exp = stmt
            .expression
            .as_any()
            .downcast_ref::<PrefixExpression>()
            .expect(&format!(
                "stmt is not ast.PrefixExpression. got={:?}",
                stmt.expression
            ));
        assert!(
            exp.operator == tt.1,
            "exp.operator is not '{}'. got={}",
            tt.1,
            exp.operator
        );
        test_literal_expression(&exp.right, &tt.2);
    }
}

fn test_integer_literal(il: &Box<dyn Expression>, value: i64) {
    let literal = il
        .as_any()
        .downcast_ref::<IntegerLiteral>()
        .expect(&format!("exp not ast.IntegerLiteral. got={:?}", il));
    assert!(
        literal.value == value,
        "literal.value not {}, got={}",
        value,
        literal.value
    );
    assert!(
        literal.token_literal() == value.to_string(),
        "literal.token_literal not {}, got={}",
        value,
        literal.token_literal()
    );
}
#[test]
fn test_parsing_infix_expressions() {
    let infix_tests: [(&str, Box<dyn Any>, &str, Box<dyn Any>); 13] = [
        ("5 + 5;", Box::new(5 as i64), "+", Box::new(5 as i64)),
        ("5 - 5;", Box::new(5 as i64), "-", Box::new(5 as i64)),
        ("5 * 5;", Box::new(5 as i64), "*", Box::new(5 as i64)),
        ("5 / 5;", Box::new(5 as i64), "/", Box::new(5 as i64)),
        ("5 > 5;", Box::new(5 as i64), ">", Box::new(5 as i64)),
        ("5 < 5;", Box::new(5 as i64), "<", Box::new(5 as i64)),
        ("5 == 5;", Box::new(5 as i64), "==", Box::new(5 as i64)),
        ("5 != 5;", Box::new(5 as i64), "!=", Box::new(5 as i64)),
        ("true == true", Box::new(true), "==", Box::new(true)),
        ("true != false", Box::new(true), "!=", Box::new(false)),
        ("false == false", Box::new(false), "==", Box::new(false)),
        ("5 + 10", Box::new(5 as i64), "+", Box::new(10 as i64)),
        ("alice * bob", Box::new("alice"), "*", Box::new("bob")),
    ];

    for tt in infix_tests.iter() {
        let mut l = Lexer::new(tt.0);
        let mut p = Parser::new(&mut l);
        let program = p.parse_program().expect("parse_program() returned None");
        check_parser_errors(&mut p);
        assert!(
            program.statements.len() == 1,
            "program has not enough statements. got={}",
            program.statements.len()
        );
        let stmt = program.statements[0]
            .as_any()
            .downcast_ref::<ExpressionStmt>()
            .expect(&format!(
                "program.statements[0] is not ast.ExpressionStmt. got={:?}",
                program.statements[0]
            ));
        let exp = stmt
            .expression
            .as_any()
            .downcast_ref::<InfixExpression>()
            .expect(&format!(
                "stmt is not ast.InfixExpression. got={:?}",
                stmt.expression
            ));
        test_literal_expression(&exp.left, &tt.1);
        assert!(
            exp.operator == tt.2,
            "exp.operator is not '{}'. got={:?}",
            tt.2,
            exp.operator
        );
        test_literal_expression(&exp.right, &tt.3);
    }
}

#[test]
fn test_operator_precedence_parsing() {
    let tests = [
        ("-a * b", "((-a) * b)"),
        ("!-a", "(!(-a))"),
        ("a + b + c", "((a + b) + c)"),
        ("a + b - c", "((a + b) - c)"),
        ("a * b * c", "((a * b) * c)"),
        ("a * b / c", "((a * b) / c)"),
        ("a + b / c", "(a + (b / c))"),
        ("a + b * c + d / e - f", "(((a + (b * c)) + (d / e)) - f)"),
        ("3 + 4; -5 * 5", "(3 + 4)((-5) * 5)"),
        ("5 > 4 == 3 < 4", "((5 > 4) == (3 < 4))"),
        ("5 < 4 != 3 > 4", "((5 < 4) != (3 > 4))"),
        (
            "3 + 4 * 5 == 3 * 1 + 4 * 5",
            "((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))",
        ),
        ("true", "true"),
        ("false", "false"),
        ("3 > 5 == false", "((3 > 5) == false)"),
        ("3 < 5 == true", "((3 < 5) == true)"),
        ("1 + (2 + 3) + 4", "((1 + (2 + 3)) + 4)"),
        ("(5 + 5) * 2", "((5 + 5) * 2)"),
        ("2 / (5 + 5)", "(2 / (5 + 5))"),
        ("-(5 + 5)", "(-(5 + 5))"),
        ("!(true == true)", "(!(true == true))"),
        ("a + add(b * c) + d", "((a + add((b * c))) + d)"),
        (
            "add(a, b, 1, 2 * 3, 4 + 5, add(6, 7 * 8))",
            "add(a, b, 1, (2 * 3), (4 + 5), add(6, (7 * 8)))",
        ),
        (
            "add(a + b + c * d / f + g)",
            "add((((a + b) + ((c * d) / f)) + g))",
        ),
    ];
    for tt in tests.iter() {
        let mut l = Lexer::new(tt.0);
        let mut p = Parser::new(&mut l);
        let program = p.parse_program().expect("parse_program() returned None");
        check_parser_errors(&mut p);
        let actual = program.string();
        assert!(actual == tt.1, "expected={}, got={}", tt.1, actual);
    }
}

fn test_identifier(exp: &Box<dyn Expression>, value: &str) {
    let ident = exp
        .as_any()
        .downcast_ref::<Identifier>()
        .expect(&format!("exp not ast.Identifier. got={:?}", exp));

    assert!(
        ident.value == value,
        "ident.value not {}. got={}",
        value,
        ident.value
    );
    assert!(
        ident.token_literal() == value,
        "ident.token_literal not {}. got={}",
        value,
        ident.token_literal()
    );
}

fn test_literal_expression(exp: &Box<dyn Expression>, expected: &Box<dyn Any>) {
    match expected.downcast_ref::<i64>() {
        Some(i64_value) => {
            test_integer_literal(exp, *i64_value);
        }
        _ => match expected.downcast_ref::<&str>() {
            Some(string_value) => {
                test_identifier(exp, &string_value);
            }
            _ => match expected.downcast_ref::<bool>() {
                Some(bool_value) => {
                    test_bool_literal(exp, *bool_value);
                }
                _ => {
                    assert!(false, "type of exp not handled. got={:?}", exp);
                }
            },
        },
    }
}

fn test_infix_expression(
    exp: &Box<dyn Expression>,
    left: Box<dyn Object>,
    operator: &str,
    right: Box<dyn Object>,
) {
    let op_exp = exp
        .as_any()
        .downcast_ref::<InfixExpression>()
        .expect(&format!("exp is not ast.OperatorExpression. got={:?}", exp));
    test_literal_expression(&op_exp.left, &left);
    assert!(
        op_exp.operator == operator,
        "exp.operator is not '{}'. got={}",
        operator,
        op_exp.operator
    );
    test_literal_expression(&op_exp.right, &right);
}

fn test_bool_literal(exp: &Box<dyn Expression>, value: bool) {
    let bo = exp
        .as_any()
        .downcast_ref::<Boolean>()
        .expect(&format!("exp not ast.Boolean. got={:?}", exp));
    assert!(
        bo.value == value,
        "bo.value not {}. got={}",
        value,
        bo.value
    );
    assert!(
        bo.token_literal() == format!("{}", value),
        "bo.token_literal not {}. got={}",
        value,
        bo.token_literal()
    );
}

#[test]
fn test_if_expression() {
    let input = "if (x < y) { x }";
    let mut l = Lexer::new(input);
    let mut p = Parser::new(&mut l);
    let program = p.parse_program().expect("parse_program() returned None");
    check_parser_errors(&p);
    assert!(
        program.statements.len() == 1,
        "program.statements does not contain 1 statements. got={}",
        program.statements.len()
    );
    let stmt = program.statements[0]
        .as_any()
        .downcast_ref::<ExpressionStmt>()
        .expect(&format!(
            "program.statement[0] is not ast.ExpressionStmt. got={:?}",
            program.statements[0]
        ));
    let exp = stmt
        .expression
        .as_any()
        .downcast_ref::<IfExpression>()
        .expect(&format!(
            "stmt.expression is not ast.IfExpression. got={:?}",
            stmt.expression
        ));
    test_infix_expression(&exp.condition, Box::new("x"), "<", Box::new("y"));
    assert!(
        exp.consequence.statements.len() == 1,
        "consequence is not 1 statements. got={}",
        exp.consequence.statements.len()
    );
    let consequence = exp.consequence.statements[0]
        .as_any()
        .downcast_ref::<ExpressionStmt>()
        .expect(&format!(
            "statements[0] is not ast.ExpressionStmt. got={:?}",
            exp.consequence.statements[0]
        ));
    test_identifier(&consequence.expression, "x");
    match &exp.alternative {
        Some(alternative) => {
            assert!(
                false,
                "exp.alternative.statements was not None, got={:?}",
                alternative
            );
        }
        _ => {}
    }
}

#[test]
fn test_if_else_expression() {
    let input = "if (x < y) { x } else { y }";
    let mut l = Lexer::new(input);
    let mut p = Parser::new(&mut l);
    let program = p.parse_program().expect("parse_program() returned None");
    check_parser_errors(&p);
    assert!(
        program.statements.len() == 1,
        "program.statements does not contain 1 statements. got={}",
        program.statements.len()
    );
    let stmt = program.statements[0]
        .as_any()
        .downcast_ref::<ExpressionStmt>()
        .expect(&format!(
            "program.statement[0] is not ast.ExpressionStmt. got={:?}",
            program.statements[0]
        ));
    let exp = stmt
        .expression
        .as_any()
        .downcast_ref::<IfExpression>()
        .expect(&format!(
            "stmt.expression is not ast.IfExpression. got={:?}",
            stmt.expression
        ));
    test_infix_expression(&exp.condition, Box::new("x"), "<", Box::new("y"));
    assert!(
        exp.consequence.statements.len() == 1,
        "consequence is not 1 statements. got={}",
        exp.consequence.statements.len()
    );
    let consequence = exp.consequence.statements[0]
        .as_any()
        .downcast_ref::<ExpressionStmt>()
        .expect(&format!(
            "statements[0] is not ast.ExpressionStmt. got={:?}",
            exp.consequence.statements[0]
        ));
    test_identifier(&consequence.expression, "x");
    match &exp.alternative {
        Some(alternative) => {
            assert!(
                alternative.statements.len() == 1,
                "alternative is not 1 statements. got={}",
                alternative.statements.len()
            );

            let a = alternative.statements[0]
                .as_any()
                .downcast_ref::<ExpressionStmt>()
                .expect(&format!(
                    "statements[0] is not ast.ExpressionStmt. got={:?}",
                    alternative.statements[0],
                ));
            test_identifier(&a.expression, "y");
        }
        _ => {}
    }
}

#[test]
fn test_function_literal_parsing() {
    let input = "fn(x, y) { x + y; }";
    let mut l = Lexer::new(input);
    let mut p = Parser::new(&mut l);
    let program = p.parse_program().expect("parse_program() returned None");
    check_parser_errors(&p);
    assert!(
        program.statements.len() == 1,
        "program.statements does not contain 1 statements. got={}",
        program.statements.len()
    );
    let stmt = program.statements[0]
        .as_any()
        .downcast_ref::<ExpressionStmt>()
        .expect(&format!(
            "program.statement[0] is not ast.ExpressionStmt. got={:?}",
            program.statements[0]
        ));
    let function = stmt
        .expression
        .as_any()
        .downcast_ref::<FunctionLiteral>()
        .expect(&format!(
            "stmt.expression is not ast.FunctionLiteral. got={:?}",
            stmt.expression
        ));
    assert!(
        function.parameters.len() == 2,
        "function literal parameters wrong. want 2, got={}",
        function.parameters.len()
    );
    test_literal_expression(
        &(Box::new(function.parameters[0].clone()) as Box<dyn Expression>),
        &(Box::new("x") as Box<dyn Any>),
    );
    test_literal_expression(
        &(Box::new(function.parameters[1].clone()) as Box<dyn Expression>),
        &(Box::new("y") as Box<dyn Any>),
    );

    assert!(
        function.body.statements.len() == 1,
        "function.body.statements has not 1 statements. got={}",
        function.body.statements.len()
    );

    let body_stmt = function.body.statements[0]
        .as_any()
        .downcast_ref::<ExpressionStmt>()
        .expect(&format!(
            "function body stmt is not ast.ExpressionStmt. got={:?}",
            function.body.statements[0]
        ));
    test_infix_expression(&body_stmt.expression, Box::new("x"), "+", Box::new("y"));
}

#[test]
fn test_function_parameter_parsing() {
    let tests = [
        ("fn() {};", vec![]),
        ("fn(x) {};", vec!["x"]),
        ("fn(x, y, z) {};", vec!["x", "y", "z"]),
    ];
    for tt in tests.iter() {
        let mut l = Lexer::new(tt.0);
        let mut p = Parser::new(&mut l);
        let program = p.parse_program().unwrap();
        check_parser_errors(&p);

        let stmt = program.statements[0]
            .as_ref()
            .as_any()
            .downcast_ref::<ExpressionStmt>()
            .unwrap();

        let function = stmt
            .expression
            .as_any()
            .downcast_ref::<FunctionLiteral>()
            .unwrap();

        assert!(
            function.parameters.len() == tt.1.len(),
            "length parameters wrong. want {}, got={}",
            tt.1.len(),
            function.parameters.len()
        );

        for (i, ident) in tt.1.iter().enumerate() {
            test_literal_expression(
                &(Box::new(function.parameters[i].clone()) as Box<dyn Expression>),
                &(Box::new(ident.clone()) as Box<dyn Any>),
            );
        }
    }
}

#[test]
fn test_call_expression_parsing() {
    let input = "add(1, 2 * 3, 4 + 5);";
    let mut l = Lexer::new(input);
    let mut p = Parser::new(&mut l);
    let program = p.parse_program().unwrap();
    check_parser_errors(&p);

    assert!(
        program.statements.len() == 1,
        "program.statements does not contain 1 statements. got={}",
        program.statements.len()
    );

    let stmt = program.statements[0]
        .as_ref()
        .as_any()
        .downcast_ref::<ExpressionStmt>()
        .unwrap();

    let exp = stmt
        .expression
        .as_any()
        .downcast_ref::<CallExpression>()
        .expect(&format!(
            "stmt.expression is not ast.CallExpression. got={:?}",
            stmt.expression
        ));

    test_identifier(&exp.function, "add");

    assert!(
        exp.arguments.len() == 3,
        "wrong length of arguments. got={}",
        exp.arguments.len()
    );

    test_literal_expression(&exp.arguments[0], &(Box::new(1 as i64) as Box<dyn Any>));
    test_infix_expression(
        &exp.arguments[1],
        Box::new(2 as i64),
        "*",
        Box::new(3 as i64),
    );
    test_infix_expression(
        &exp.arguments[2],
        Box::new(4 as i64),
        "+",
        Box::new(5 as i64),
    );
}
