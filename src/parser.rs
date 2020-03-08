use super::ast::*;
use super::lexer::*;
use std::collections::HashMap;

pub struct Parser<'a> {
    l: &'a mut Lexer<'a>,
    cur_token: Token,
    peek_token: Token,
    errors: Vec<Box<String>>,
    precedences: HashMap<TokenType, Precedence>,
}

#[derive(PartialOrd, PartialEq, Copy, Clone)]
#[allow(dead_code)]
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
            errors: Vec::new(),
            precedences: HashMap::new(),
        };
        p.precedences.insert(TokenType::EQ, Precedence::EQUALS);
        p.precedences.insert(TokenType::NOTEQ, Precedence::EQUALS);
        p.precedences
            .insert(TokenType::LT, Precedence::LESSGEREATER);
        p.precedences
            .insert(TokenType::GT, Precedence::LESSGEREATER);
        p.precedences.insert(TokenType::PLUS, Precedence::SUM);
        p.precedences.insert(TokenType::MINUS, Precedence::SUM);
        p.precedences.insert(TokenType::SLASH, Precedence::PRODUCT);
        p.precedences
            .insert(TokenType::ASTERISK, Precedence::PRODUCT);

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

    #[allow(dead_code)]
    pub fn get_errors(&self) -> &Vec<Box<String>> {
        &self.errors
    }
    fn peek_error(&mut self, t: TokenType) {
        let msg = format!(
            "expected next token to be {:?}, got {:?} instead",
            t, self.peek_token.tk_type
        );
        self.errors.push(Box::new(msg));
    }

    fn peek_precedence(&self) -> Precedence {
        match self.precedences.get(&self.peek_token.tk_type) {
            Some(p) => *p,
            _ => Precedence::LOWEST,
        }
    }
    fn cur_precedence(&self) -> Precedence {
        match self.precedences.get(&self.cur_token.tk_type) {
            Some(p) => *p,
            _ => Precedence::LOWEST,
        }
    }

    pub fn parse_program(&mut self) -> Option<Program> {
        println!("parse_program: {:?}", self.cur_token);
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
        println!("parse_statement: {:?}", self.cur_token);
        match self.cur_token.tk_type {
            TokenType::LET => self.parse_let_statement(),
            TokenType::RETURN => self.parse_return_statement(),
            _ => self.parse_expression_statement(),
        }
    }

    fn parse_let_statement(&mut self) -> Option<Box<dyn Statement>> {
        println!("parse_let_statement: {:?}", self.cur_token);
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
        println!("parse_return_statement: {:?}", self.cur_token);
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
        println!("parse_expression_statement: {:?}", self.cur_token);
        let mut stmt = ExpressionStmt {
            token: self.cur_token.clone(),
            expression: None,
        };

        stmt.expression = self.parse_expression(Precedence::LOWEST);

        if self.peek_token_is(TokenType::SEMICOLON) {
            self.next_token();
        }
        Some(Box::new(stmt))
    }

    fn parse_expression(&mut self, precedence: Precedence) -> Option<Box<dyn Expression>> {
        println!("parse_expression: {:?}", self.cur_token);
        let mut left_exp: Option<Box<dyn Expression>>;
        match self.cur_token.tk_type {
            TokenType::IDENT => {
                left_exp = self.parse_identifier();
            }
            TokenType::INT => {
                left_exp = self.parse_integer_literal();
            }
            TokenType::BANG => {
                left_exp = self.parse_prefix_expression();
            }
            TokenType::MINUS => {
                left_exp = self.parse_prefix_expression();
            }
            _ => {
                self.no_prefix_parse_fn_error(self.cur_token.tk_type);
                return None;
            }
        };

        while !self.peek_token_is(TokenType::SEMICOLON) && precedence < self.peek_precedence() {
            match self.peek_token.tk_type {
                TokenType::PLUS => {
                    self.next_token();
                    left_exp = self.parse_infix_expression(left_exp);
                }
                TokenType::MINUS => {
                    self.next_token();
                    left_exp = self.parse_infix_expression(left_exp);
                }
                TokenType::SLASH => {
                    self.next_token();
                    left_exp = self.parse_infix_expression(left_exp);
                }
                TokenType::ASTERISK => {
                    self.next_token();
                    left_exp = self.parse_infix_expression(left_exp);
                }
                TokenType::EQ => {
                    self.next_token();
                    left_exp = self.parse_infix_expression(left_exp);
                }
                TokenType::NOTEQ => {
                    self.next_token();
                    left_exp = self.parse_infix_expression(left_exp);
                }
                TokenType::LT => {
                    self.next_token();
                    left_exp = self.parse_infix_expression(left_exp);
                }
                TokenType::GT => {
                    self.next_token();
                    left_exp = self.parse_infix_expression(left_exp);
                }
                _ => {
                    return left_exp;
                }
            }
        }
        left_exp
    }

    fn parse_identifier(&mut self) -> Option<Box<dyn Expression>> {
        println!("parse_identifier: {:?}", self.cur_token);
        Some(Box::new(Identifier {
            token: self.cur_token.clone(),
            value: self.cur_token.literal.clone(),
        }))
    }

    fn parse_integer_literal(&mut self) -> Option<Box<dyn Expression>> {
        println!("parse_integer_literal: {:?}", self.cur_token);
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
                self.errors.push(Box::new(msg));
                None
            }
        }
    }

    fn no_prefix_parse_fn_error(&mut self, t: TokenType) {
        let msg = format!("no prefix parse function for {:?} found", t);
        self.errors.push(Box::new(msg));
    }

    fn parse_prefix_expression(&mut self) -> Option<Box<dyn Expression>> {
        let mut expression = PrefixExpression {
            token: self.cur_token.clone(),
            operator: self.cur_token.literal.clone(),
            right: None,
        };
        self.next_token();
        expression.right = self.parse_expression(Precedence::PREFIX);
        Some(Box::new(expression))
    }

    fn parse_infix_expression(
        &mut self,
        left: Option<Box<dyn Expression>>,
    ) -> Option<Box<dyn Expression>> {
        let mut expression = InfixExpression {
            token: self.cur_token.clone(),
            left: left,
            operator: self.cur_token.literal.clone(),
            right: None,
        };

        let precedence = self.cur_precedence();
        self.next_token();
        expression.right = self.parse_expression(precedence);
        Some(Box::new(expression))
    }
}
