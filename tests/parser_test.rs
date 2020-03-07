extern crate waiir;

use waiir::ast::*;
use waiir::lexer::*;
use waiir::parser::*;

fn check_parser_errors(p: &Parser) {
    let errors = &p.get_errors().borrow();
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
    let input = "
    let x = 5;
    let y = 10;
    let foobar = 838383;
    ";
    let l = Lexer::new(input);
    let p = Parser::new(l);
    let program = p.parse_program().expect("parse_program() returned None");
    check_parser_errors(&p);
    assert!(
        program.statements.len() == 3,
        "program.statements does not contain 3 statements. got={}",
        program.statements.len()
    );
    let tests = ["x", "y", "foobar"];
    for (i, name) in tests.iter().enumerate() {
        let stmt = &program.statements[i];
        test_let_statement(stmt, name);
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
        .expect(&format!("s not ast.LetStatement. got={}", s));

    assert!(
        let_stmt.name.as_ref().unwrap().value == *name,
        "let_stmt.name.value not '{}'. got={}",
        name,
        let_stmt.name.as_ref().unwrap().value
    );
    assert!(
        let_stmt.name.as_ref().unwrap().token_literal() == name,
        "s.name not '{}'. got={}",
        name,
        let_stmt.name.as_ref().unwrap().token_literal()
    );
}

#[test]
fn test_return_statements() {
    let input = "
        return 5;
        return 10;
        return 993322;
        ";
    let l = Lexer::new(&input);
    let p = Parser::new(l);

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
            .expect(&format!("stmt not ReturnStatment. got={}", stmt));
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
            name: Some(Identifier {
                token: Token {
                    tk_type: TokenType::IDENT,
                    literal: String::from("myVar"),
                },
                value: String::from("myVar"),
            }),
            value: Some(Box::new(Identifier {
                token: Token {
                    tk_type: TokenType::IDENT,
                    literal: String::from("anotherVar"),
                },
                value: String::from("anotherVar"),
            })),
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
    let l = Lexer::new(input);
    let p = Parser::new(l);
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
            "program.statements[0] is not ExpressionStmt. got={}",
            program.statements[0]
        ));
    match &stmt.expression {
        Some(expression) => {
            let ident = expression
                .as_any()
                .downcast_ref::<Identifier>()
                .expect(&format!("exp not Identifier. got={}", expression));
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
        _ => {
            assert!(false, "exp not Identifier. got={}", stmt);
        }
    }
}

#[test]
fn test_integer_literal_expression() {
    let input = "5;";
    let l = Lexer::new(input);
    let p = Parser::new(l);
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
            "program.statements[0] is not ast.ExpressionStmt. got={}",
            program.statements[0]
        ));
    match &stmt.expression {
        Some(expression) => {
            test_integer_literal(expression, 5);
        }
        _ => {
            assert!(false, "exp not ast.IntegerLiteral. got={}", stmt);
        }
    }
}

#[test]
fn test_parsing_prefix_expressions() {
    let prefix_tests = [("!5;", "!", 5), ("-15;", "-", 15)];
    for tt in prefix_tests.iter() {
        let l = Lexer::new(tt.0);
        let p = Parser::new(l);
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
                "program.statements[0] is not ast.ExpressionStmt. got={}",
                program.statements[0]
            ));
        match &stmt.expression {
            Some(expression) => {
                let exp = expression
                    .as_any()
                    .downcast_ref::<PrefixExpression>()
                    .expect(&format!(
                        "stmt is not ast.PrefixExpression. got={}",
                        expression
                    ));
                assert!(
                    exp.operator == tt.1,
                    "exp.operator is not '{}'. got={}",
                    tt.1,
                    exp.operator
                );
                let right = exp.right.as_ref().expect("exp.right is None");
                test_integer_literal(&right, tt.2);
            }
            _ => {
                assert!(false, "exp not ast.PrefixExpression. got={}", stmt);
            }
        }
    }
}

fn test_integer_literal(il: &Box<dyn Expression>, value: i64) {
    let literal = il
        .as_any()
        .downcast_ref::<IntegerLiteral>()
        .expect(&format!("exp not ast.IntegerLiteral. got={}", il));
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
