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
            cur_token: Token::new(TokenType::Illegal, 0 as char),
            peek_token: Token::new(TokenType::Illegal, 0 as char),
            errors: Vec::new(),
            precedences: HashMap::new(),
        };
        p.precedences.insert(TokenType::Eq, Precedence::EQUALS);
        p.precedences.insert(TokenType::NotEq, Precedence::EQUALS);
        p.precedences
            .insert(TokenType::Lt, Precedence::LESSGEREATER);
        p.precedences
            .insert(TokenType::Gt, Precedence::LESSGEREATER);
        p.precedences.insert(TokenType::Plus, Precedence::SUM);
        p.precedences.insert(TokenType::Minus, Precedence::SUM);
        p.precedences.insert(TokenType::Slash, Precedence::PRODUCT);
        p.precedences
            .insert(TokenType::Asterisk, Precedence::PRODUCT);
        p.precedences.insert(TokenType::Lparen, Precedence::CALL);

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
        while self.cur_token.tk_type != TokenType::Eof {
            let stmt = self.parse_statement();
            if stmt.is_some() {
                program.statements.push(stmt.unwrap());
            }
            self.next_token();
        }
        Some(program)
    }

    fn parse_statement(&mut self) -> Option<Box<dyn Statement>> {
        println!("parse_statement: {:?}", self.cur_token);
        match self.cur_token.tk_type {
            TokenType::Let => self.parse_let_statement(),
            TokenType::Return => self.parse_return_statement(),
            _ => self.parse_expression_statement(),
        }
    }

    fn parse_let_statement(&mut self) -> Option<Box<dyn Statement>> {
        println!("parse_let_statement: {:?}", self.cur_token);
        let token = self.cur_token.clone();

        if !self.expect_peek(TokenType::Ident) {
            return None;
        }

        let name_token = self.cur_token.clone();
        let name_value = self.cur_token.literal.clone();

        if !self.expect_peek(TokenType::Assign) {
            return None;
        }

        self.next_token();

        let value = self.parse_expression(Precedence::LOWEST);
        if value.is_none() {
            return None;
        }

        let stmt = LetStatement {
            token: token,
            name: Identifier {
                token: name_token,
                value: name_value,
            },
            value: value.unwrap(),
        };

        while !self.cur_token_is(TokenType::Semicolon) {
            self.next_token();
        }
        Some(Box::new(stmt))
    }
    fn parse_return_statement(&mut self) -> Option<Box<dyn Statement>> {
        println!("parse_return_statement: {:?}", self.cur_token);
        let token = self.cur_token.clone();
        self.next_token();

        let return_value = self.parse_expression(Precedence::LOWEST);
        if return_value.is_none() {
            return None;
        }
        let stmt = ReturnStatement {
            token: token,
            return_value: return_value.unwrap(),
        };

        if self.peek_token_is(TokenType::Semicolon) {
            self.next_token();
        }

        Some(Box::new(stmt))
    }

    fn parse_expression_statement(&mut self) -> Option<Box<dyn Statement>> {
        println!("parse_expression_statement: {:?}", self.cur_token);
        let token = self.cur_token.clone();
        let expression = self.parse_expression(Precedence::LOWEST);
        if expression.is_none() {
            return None;
        }
        let stmt = ExpressionStmt {
            token: token,
            expression: expression.unwrap(),
        };

        if self.peek_token_is(TokenType::Semicolon) {
            self.next_token();
        }
        Some(Box::new(stmt))
    }

    fn parse_expression(&mut self, precedence: Precedence) -> Option<Box<dyn Expression>> {
        println!("parse_expression: {:?}", self.cur_token);
        let mut left_exp: Option<Box<dyn Expression>>;
        match self.cur_token.tk_type {
            TokenType::Ident => {
                left_exp = self.parse_identifier();
            }
            TokenType::Int => {
                left_exp = self.parse_integer_literal();
            }
            TokenType::Bang => {
                left_exp = self.parse_prefix_expression();
            }
            TokenType::Minus => {
                left_exp = self.parse_prefix_expression();
            }
            TokenType::True => {
                left_exp = self.parse_boolean();
            }
            TokenType::False => {
                left_exp = self.parse_boolean();
            }
            TokenType::Lparen => {
                left_exp = self.parse_grouped_expression();
            }
            TokenType::If => {
                left_exp = self.parse_if_expression();
            }
            TokenType::Function => {
                left_exp = self.parse_function_literal();
            }
            _ => {
                self.no_prefix_parse_fn_error(self.cur_token.tk_type);
                return None;
            }
        };

        if left_exp.is_none() {
            return None;
        }

        while !self.peek_token_is(TokenType::Semicolon) && precedence < self.peek_precedence() {
            match self.peek_token.tk_type {
                TokenType::Plus => {
                    self.next_token();
                    left_exp = self.parse_infix_expression(left_exp.unwrap());
                }
                TokenType::Minus => {
                    self.next_token();
                    left_exp = self.parse_infix_expression(left_exp.unwrap());
                }
                TokenType::Slash => {
                    self.next_token();
                    left_exp = self.parse_infix_expression(left_exp.unwrap());
                }
                TokenType::Asterisk => {
                    self.next_token();
                    left_exp = self.parse_infix_expression(left_exp.unwrap());
                }
                TokenType::Eq => {
                    self.next_token();
                    left_exp = self.parse_infix_expression(left_exp.unwrap());
                }
                TokenType::NotEq => {
                    self.next_token();
                    left_exp = self.parse_infix_expression(left_exp.unwrap());
                }
                TokenType::Lt => {
                    self.next_token();
                    left_exp = self.parse_infix_expression(left_exp.unwrap());
                }
                TokenType::Gt => {
                    self.next_token();
                    left_exp = self.parse_infix_expression(left_exp.unwrap());
                }
                TokenType::Lparen => {
                    self.next_token();
                    left_exp = self.parse_call_expression(left_exp.unwrap());
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
        println!("parse_prefix_expression: {:?}", self.cur_token);
        let token = self.cur_token.clone();
        let operator = self.cur_token.literal.clone();
        self.next_token();
        let right = self.parse_expression(Precedence::PREFIX);
        if right.is_none() {
            return None;
        }
        let expression = PrefixExpression {
            token: token,
            operator: operator,
            right: right.unwrap(),
        };

        Some(Box::new(expression))
    }

    fn parse_infix_expression(&mut self, left: Box<dyn Expression>) -> Option<Box<dyn Expression>> {
        println!("parse_infix_expression: {:?}", self.cur_token);
        let token = self.cur_token.clone();
        let operator = self.cur_token.literal.clone();

        let precedence = self.cur_precedence();
        self.next_token();

        let right = self.parse_expression(precedence);
        if right.is_none() {
            return None;
        }
        let expression = InfixExpression {
            token: token,
            left: left,
            operator: operator,
            right: right.unwrap(),
        };

        Some(Box::new(expression))
    }

    fn parse_boolean(&mut self) -> Option<Box<dyn Expression>> {
        println!("parse_boolean: {:?}", self.cur_token);
        Some(Box::new(Boolean {
            token: self.cur_token.clone(),
            value: self.cur_token_is(TokenType::True),
        }))
    }

    fn parse_grouped_expression(&mut self) -> Option<Box<dyn Expression>> {
        println!("parse_grouped_expression: {:?}", self.cur_token);
        self.next_token();
        let exp = self.parse_expression(Precedence::LOWEST);
        if !self.expect_peek(TokenType::Rparen) {
            None
        } else {
            exp
        }
    }

    fn parse_if_expression(&mut self) -> Option<Box<dyn Expression>> {
        println!("parse_if_expression: {:?}", self.cur_token);
        let token = self.cur_token.clone();
        if !self.expect_peek(TokenType::Lparen) {
            return None;
        }
        self.next_token();
        let condition = self.parse_expression(Precedence::LOWEST);
        if condition.is_none() {
            return None;
        }
        if !self.expect_peek(TokenType::Rparen) {
            return None;
        }

        if !self.expect_peek(TokenType::Lbrace) {
            return None;
        }

        let consequence = self.parse_block_statement();
        if consequence.is_none() {
            return None;
        }

        let mut expression = IfExpression {
            token: token,
            condition: condition.unwrap(),
            consequence: consequence.unwrap(),
            alternative: None,
        };

        if self.peek_token_is(TokenType::Else) {
            self.next_token();

            if !self.expect_peek(TokenType::Lbrace) {
                return None;
            }

            expression.alternative = self.parse_block_statement();
        }
        Some(Box::new(expression))
    }

    fn parse_block_statement(&mut self) -> Option<BlockStatement> {
        println!("parse_block_statement: {:?}", self.cur_token);
        let mut block = BlockStatement {
            token: self.cur_token.clone(),
            statements: Vec::new(),
        };

        self.next_token();

        while !self.cur_token_is(TokenType::Rbrace) && !self.cur_token_is(TokenType::Eof) {
            let stmt = self.parse_statement();
            match stmt {
                Some(s) => {
                    block.statements.push(s);
                }
                _ => {}
            }
            self.next_token();
        }
        Some(block)
    }

    fn parse_function_literal(&mut self) -> Option<Box<dyn Expression>> {
        println!("parse_function_literal: {:?}", self.cur_token);
        let token = self.cur_token.clone();
        if !self.expect_peek(TokenType::Lparen) {
            return None;
        }

        let parameters = self.parse_function_parameters();

        if !self.expect_peek(TokenType::Lbrace) {
            return None;
        }
        let body = self.parse_block_statement();
        if body.is_none() {
            return None;
        }
        let lit = FunctionLiteral {
            token: token,
            parameters: parameters,
            body: body.unwrap(),
        };

        Some(Box::new(lit))
    }

    fn parse_function_parameters(&mut self) -> Vec<Identifier> {
        println!("parse_function_parameters: {:?}", self.cur_token);
        let mut identfiers = Vec::new();
        if self.peek_token_is(TokenType::Rparen) {
            self.next_token();
            return identfiers;
        }

        self.next_token();

        let ident = Identifier {
            token: self.cur_token.clone(),
            value: self.cur_token.literal.clone(),
        };
        identfiers.push(ident);

        while self.peek_token_is(TokenType::Comma) {
            self.next_token();
            self.next_token();
            let ident = Identifier {
                token: self.cur_token.clone(),
                value: self.cur_token.literal.clone(),
            };
            identfiers.push(ident);
        }

        if !self.expect_peek(TokenType::Rparen) {
            return Vec::new();
        }
        identfiers
    }

    fn parse_call_expression(
        &mut self,
        function: Box<dyn Expression>,
    ) -> Option<Box<dyn Expression>> {
        println!("parse_call_expression: {:?}", self.cur_token);
        let exp = CallExpression {
            token: self.cur_token.clone(),
            function: function,
            arguments: self.parse_call_arguments(),
        };
        Some(Box::new(exp))
    }

    fn parse_call_arguments(&mut self) -> Vec<Box<dyn Expression>> {
        println!("parse_call_arguments: {:?}", self.cur_token);
        let mut args: Vec<Box<dyn Expression>> = Vec::new();

        if self.peek_token_is(TokenType::Rparen) {
            self.next_token();
            return args;
        }

        self.next_token();
        let a = self.parse_expression(Precedence::LOWEST);
        if a.is_none() {
            return Vec::new();
        }
        args.push(a.unwrap());

        while self.peek_token_is(TokenType::Comma) {
            self.next_token();
            self.next_token();
            let a = self.parse_expression(Precedence::LOWEST);
            if a.is_none() {
                return Vec::new();
            }
            args.push(a.unwrap());
        }

        if !self.expect_peek(TokenType::Rparen) {
            return Vec::new();
        }

        args
    }
}
