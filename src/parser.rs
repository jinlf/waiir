use super::ast::*;
use super::lexer::*;

struct Parser<'a> {
    l: &'a mut Lexer,
    cur_token: Token,
    peek_token: Token,
    errors:Vec<Box<String>>,
}

impl<'a> Parser<'a> {
    fn new(lex: &'a mut Lexer) -> Parser {
        let mut p = Parser {
            l: lex,
            cur_token: Token::new(TokenType::ILLEGAL, 0 as char),
            peek_token: Token::new(TokenType::ILLEGAL, 0 as char),
            errors: Vec::new(),
        };
        p.next_token();
        p.next_token();
        p
    }

    fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.l.next_token();
    }

    fn parse_program(&mut self) -> Option<Program> {
        let mut program = Program {
            statements: Vec::new(),
        };
        while self.cur_token.tk_type != TokenType::EOF {
            match self.parse_statement() {
                Some(stmt) => {
                    program.statements.push(stmt);
                }
                _ => {}
            }
            self.next_token();
        }
        Some(program)
    }

    fn parse_statement(&mut self) -> Option<Box<dyn Statement>> {
        match self.cur_token.tk_type {
            TokenType::LET => self.parse_let_statement(),
            _ => None,
        }
    }

    fn parse_let_statement(&mut self) -> Option<Box<dyn Statement>> {
        let mut stmt = LetStatement {
            token: self.cur_token.clone(),
            name: None,
            value: None,
        };

        if !self.expect_peek(TokenType::IDENT) {
            return None;
        }
        stmt.name = Some(Identifier {
            token: self.cur_token.clone(),
            value: self.cur_token.literal.clone(),
        });

        //TODO

        if !self.expect_peek(TokenType::ASSIGN) {
            return None;
        }
        while !self.cur_token_is(TokenType::SEMICOLON) {
            self.next_token();
        }
        return Some(Box::new(stmt) as Box<dyn Statement>);
    }

    fn cur_token_is(&self, t: TokenType) -> bool {
        self.cur_token.tk_type == t
    }
    fn peek_token_is(&self, t: TokenType) -> bool {
        self.peek_token.tk_type == t
    }
    fn expect_peek(&mut self, t: TokenType) -> bool {
        if self.peek_token_is(t) {
            self.next_token();
            true
        } else {
            self.peek_error(t);
            false
        }
    }

    fn Errors(&self) -> &Vec<Box<String>> {
        &self.errors
    }
    fn peek_error(&mut self, t:TokenType) {
        let msg = format!("expected next token to be {:?}, got {:?} instead",
            t, self.peek_token.tk_type);
        self.errors.push(Box::new(msg));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_let_statements() {
        let input = "
    let x = 5;
    let y = 10;
    let foobar = 838383;
    ";
        let mut l = Lexer::new(input);
        let mut p = Parser::new(&mut l);
        match p.parse_program() {
            Some(program) => {
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
            _ => {
                check_parser_errors(&p);
                assert!(false, "parse_program() returned nil");
            }
        }
    }

    fn check_parser_errors(p:&Parser) {
        let errors = &p.errors;
        if errors.len() == 0 {
            return
        }
        println!("parser has {} errors", errors.len());
        for msg in errors.iter() {
            println!("parser error: {}", *msg);
        }
        assert!(false, "parser error!!!");

    }

    fn test_let_statement(s: &Box<dyn Statement>, name: &str){
        assert!(
            s.token_literal() == "let",
            "s.token_literal not 'let'. got={}",
            s.token_literal()
        );
        let let_stmt = s.as_any().downcast_ref::<LetStatement>();
        match let_stmt {
            Some(let_stmt) => match &let_stmt.name {
                Some(identifier) => {
                    assert!(
                        identifier.value == *name,
                        "let_stmt.name.value not '{}'. got={}",
                        name,
                        identifier.value
                    );
                    assert!(
                        identifier.token_literal() == name,
                        "s.name not '{}'. got={}",
                        name,
                        identifier.token_literal()
                    );
                }
                _ => {}
            },
            _ => assert!(false, "s not ast.LetStatement. got={}", s),
        }
    }
}
