use super::ast::*;
use super::lexer::*;
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

type PrefixParseFn = dyn Fn(&Parser) -> Option<Box<dyn Expression>>;
type InfixParseFn = dyn Fn(&mut Parser, dyn Expression) -> Option<Box<dyn Expression>>;

pub struct Parser<'a> {
    l: &'a mut Lexer<'a>,
    cur_token: Token,
    peek_token: Token,
    errors: Rc<RefCell<Vec<Box<String>>>>,
    prefix_parse_fns: HashMap<TokenType, Box<PrefixParseFn>>,
    infix_parse_fns: HashMap<TokenType, Box<InfixParseFn>>,
}

enum Precedence {
    LOWEST,
    EQUALS,
    LESSGEREATER,
    SUM,
    PRODUCT,
    PREFIX,
    CALL,
}

impl<'a> Parser<'a> {
    pub fn new(lex: &'a mut Lexer<'a>) -> Parser<'a> {
        let mut p = Parser {
            l: lex,
            cur_token: Token::new(TokenType::ILLEGAL, 0 as char),
            peek_token: Token::new(TokenType::ILLEGAL, 0 as char),
            errors: Rc::new(RefCell::new(Vec::new())),
            prefix_parse_fns: HashMap::new(),
            infix_parse_fns: HashMap::new(),
        };
        p.register_prefix(
            TokenType::IDENT,
            Box::new(|parser: &Parser| Parser::parse_identifier(parser)),
        );
        p.register_prefix(
            TokenType::INT,
            Box::new(|parser: &Parser| Parser::parse_integer_literal(parser)),
        );
        p.next_token();
        p.next_token();
        p
    }

    fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.l.next_token();
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

    pub fn get_errors(&self) -> &RefCell<Vec<Box<String>>> {
        &self.errors
    }
    fn peek_error(&mut self, t: TokenType) {
        let msg = format!(
            "expected next token to be {:?}, got {:?} instead",
            t, self.peek_token.tk_type
        );
        self.errors.borrow_mut().push(Box::new(msg));
    }

    fn register_prefix(&mut self, token_type: TokenType, func: Box<PrefixParseFn>) {
        println!("register_prefix for {:?}", token_type);
        self.prefix_parse_fns.insert(token_type, func);
    }
    fn register_infix(&mut self, token_type: TokenType, func: Box<InfixParseFn>) {
        self.infix_parse_fns.insert(token_type, func);
    }

    pub fn parse_program(&mut self) -> Option<Program> {
        println!("parse_program");
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
        println!("parse_statement");
        match self.cur_token.tk_type {
            TokenType::LET => self.parse_let_statement(),
            TokenType::RETURN => self.parse_return_statement(),
            _ => self.parse_expression_statement(),
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

    fn parse_return_statement(&mut self) -> Option<Box<dyn Statement>> {
        let stmt = ReturnStatement {
            token: self.cur_token.clone(),
            return_value: None,
        };

        self.next_token();

        //TODO

        while !self.cur_token_is(TokenType::SEMICOLON) {
            self.next_token();
        }

        Some(Box::new(stmt) as Box<dyn Statement>)
    }

    fn parse_expression_statement(&mut self) -> Option<Box<dyn Statement>> {
        println!("parse_expression_statement");
        let stmt = ExpressionStmt {
            token: self.cur_token.clone(),
            expression: self.parse_expression(Precedence::LOWEST),
        };

        if self.peek_token_is(TokenType::SEMICOLON) {
            self.next_token();
        }
        Some(Box::new(stmt))
    }

    fn parse_expression(&mut self, _precedence: Precedence) -> Option<Box<dyn Expression>> {
        println!("parse_expression");
        match self.prefix_parse_fns.get(&self.cur_token.tk_type) {
            Some(prefix) => {
                println!("has callback");
                prefix(self)
            }
            _ => {
                println!("no callback");
                None
            }
        }
    }

    fn parse_identifier(&self) -> Option<Box<dyn Expression>> {
        Some(Box::new(Identifier {
            token: self.cur_token.clone(),
            value: self.cur_token.literal.clone(),
        }))
    }

    fn parse_integer_literal(&self) -> Option<Box<dyn Expression>> {
        println!("parse_integer_literal");
        let mut lit = IntegerLiteral {
            token: self.cur_token.clone(),
            value: 0,
        };

        match self.cur_token.literal.parse::<i64>() {
            Ok(v) => {
                lit.value = v;
                Some(Box::new(lit) as Box<dyn Expression>)
            }
            _ => {
                let msg = format!("could not parse {} as integer", self.cur_token.literal);
                self.errors.borrow_mut().push(Box::new(msg));
                None
            }
        }
    }
}

