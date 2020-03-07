#[derive(Debug, PartialEq, Copy, Clone, Hash)]
pub enum TokenType {
    ILLEGAL = 0,
    EOF,
    // Identifiers + literals
    IDENT, // add, foobar, x, y, ...
    INT,
    // Operators
    ASSIGN,
    PLUS,
    MINUS,
    BANG = 100,
    ASTERISK,
    SLASH,
    LT,
    GT,
    // Delimiters
    COMMA,
    SEMICOLON,
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
    // Keywords
    FUNCTION,
    LET,
    IF,
    ELSE,
    TRUE,
    FALSE,
    RETURN,
    EQ,
    NOTEQ,
}
impl std::cmp::Eq for TokenType {}

#[derive(Debug, Clone)]
pub struct Token {
    pub tk_type: TokenType,
    pub literal: String,
}

impl Token {
    pub fn new(token_type: TokenType, ch: char) -> Token {
        let mut s = String::new();
        s.push(ch);
        Token {
            tk_type: token_type,
            literal: s,
        }
    }
    fn lookup_ident(ident: &str) -> TokenType {
        match ident {
            "fn" => TokenType::FUNCTION,
            "let" => TokenType::LET,
            "if" => TokenType::IF,
            "else" => TokenType::ELSE,
            "return" => TokenType::RETURN,
            "true" => TokenType::TRUE,
            "false" => TokenType::FALSE,
            _ => TokenType::IDENT,
        }
    }
}

pub struct Lexer<'a> {
    input: &'a str,
    position: usize,      // current position in input (points to current char)
    read_position: usize, // current reading position in input (after current char)
    pub ch: char,         // current char under examination
}

const NIL: char = 0 as char;

impl<'a> Lexer<'a> {
    pub fn new(input: &str) -> Lexer {
        let mut l: Lexer = Lexer {
            input: input,
            position: 0,
            read_position: 0,
            ch: NIL,
        };
        l.read_char();
        l
    }

    pub fn next_token(&mut self) -> Token {
        let mut tok: Token;
        self.skip_whitespace();
        match self.ch {
            '=' => {
                tok = {
                    let ch = self.peek_char();
                    if ch == '=' {
                        let mut tk = Token::new(TokenType::EQ, self.ch);
                        let mut s = String::new();
                        s.push(self.ch);
                        s.push(ch);
                        tk.literal = s;
                        self.read_position += 1;
                        tk
                    } else {
                        Token::new(TokenType::ASSIGN, self.ch)
                    }
                }
            }
            ';' => tok = Token::new(TokenType::SEMICOLON, self.ch),
            '(' => tok = Token::new(TokenType::LPAREN, self.ch),
            ')' => tok = Token::new(TokenType::RPAREN, self.ch),
            ',' => tok = Token::new(TokenType::COMMA, self.ch),
            '+' => tok = Token::new(TokenType::PLUS, self.ch),
            '-' => tok = Token::new(TokenType::MINUS, self.ch),
            '!' => {
                tok = {
                    let ch = self.peek_char();
                    if ch == '=' {
                        let mut tk = Token::new(TokenType::NOTEQ, self.ch);
                        let mut s = String::new();
                        s.push(self.ch);
                        s.push(ch);
                        tk.literal = s;
                        self.read_position += 1;
                        tk
                    } else {
                        Token::new(TokenType::BANG, self.ch)
                    }
                }
            }
            '/' => tok = Token::new(TokenType::SLASH, self.ch),
            '*' => tok = Token::new(TokenType::ASTERISK, self.ch),
            '<' => tok = Token::new(TokenType::LT, self.ch),
            '>' => tok = Token::new(TokenType::GT, self.ch),
            '{' => tok = Token::new(TokenType::LBRACE, self.ch),
            '}' => tok = Token::new(TokenType::RBRACE, self.ch),
            NIL => {
                tok = Token::new(TokenType::EOF, self.ch);
                tok.literal = String::new();
            }
            _ => {
                if self.ch.is_ascii_alphabetic() {
                    tok = Token::new(TokenType::IDENT, self.ch);
                    tok.literal = String::from(self.read_identifier());
                    tok.tk_type = Token::lookup_ident(&tok.literal);
                    return tok; // need not read_char, show return now
                } else if self.ch.is_ascii_digit() {
                    tok = Token::new(TokenType::INT, self.ch);
                    tok.literal = String::from(self.read_number());
                    return tok; // need not read_char, show return now
                } else {
                    tok = Token::new(TokenType::ILLEGAL, self.ch)
                }
            }
        }
        self.read_char();
        tok
    }

    fn skip_whitespace(&mut self) {
        loop {
            match self.ch {
                ' ' | '\t' | '\n' | '\r' => {
                    self.read_char();
                }
                _ => {
                    return;
                }
            }
        }
    }

    fn read_identifier(&mut self) -> String {
        let position = self.position;
        while self.ch.is_ascii_alphabetic() {
            self.read_char();
        }
        String::from(&self.input[position..self.position])
    }

    fn read_number(&mut self) -> String {
        let position = self.position;
        while self.ch.is_ascii_digit() {
            self.read_char();
        }
        String::from(&self.input[position..self.position])
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = NIL;
        } else {
            self.ch = self.input.chars().nth(self.read_position).unwrap()
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn peek_char(&mut self) -> char {
        if self.read_position >= self.input.len() {
            NIL
        } else {
            self.input.chars().nth(self.read_position).unwrap()
        }
    }
}
