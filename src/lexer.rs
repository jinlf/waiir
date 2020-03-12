#[derive(Debug, PartialEq, Copy, Clone, Hash)]
pub enum TokenType {
    Illegal = 0,
    Eof,
    // Identifiers + literals
    Ident, // add, foobar, x, y, ...
    Int,
    // Operators
    Assign,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,
    Lt,
    Gt,
    // Delimiters
    Comma,
    Semicolon,
    Lparen,
    Rparen,
    Lbrace,
    Rbrace,
    // Keywords
    Function,
    Let,
    If,
    Else,
    True,
    False,
    Return,
    Eq,
    NotEq,
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
            "fn" => TokenType::Function,
            "let" => TokenType::Let,
            "if" => TokenType::If,
            "else" => TokenType::Else,
            "return" => TokenType::Return,
            "true" => TokenType::True,
            "false" => TokenType::False,
            _ => TokenType::Ident,
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

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = NIL;
        } else {
            self.ch = self.input.chars().nth(self.read_position).unwrap()
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn next_token(&mut self) -> Token {
        let mut tok: Token;

        self.skip_whitespace();

        match self.ch {
            '=' => {
                tok = {
                    let ch = self.peek_char();
                    if ch == '=' {
                        let tk = Token {
                            tk_type: TokenType::Eq,
                            literal: format!("{}{}", self.ch, ch),
                        };
                        self.read_position += 1;
                        tk
                    } else {
                        Token::new(TokenType::Assign, self.ch)
                    }
                }
            }
            ';' => tok = Token::new(TokenType::Semicolon, self.ch),
            '(' => tok = Token::new(TokenType::Lparen, self.ch),
            ')' => tok = Token::new(TokenType::Rparen, self.ch),
            ',' => tok = Token::new(TokenType::Comma, self.ch),
            '+' => tok = Token::new(TokenType::Plus, self.ch),
            '-' => tok = Token::new(TokenType::Minus, self.ch),
            '!' => {
                tok = {
                    let ch = self.peek_char();
                    if ch == '=' {
                        let tk = Token {
                            tk_type: TokenType::NotEq,
                            literal: format!("{}{}", self.ch, ch),
                        };
                        self.read_position += 1;
                        tk
                    } else {
                        Token::new(TokenType::Bang, self.ch)
                    }
                }
            }
            '/' => tok = Token::new(TokenType::Slash, self.ch),
            '*' => tok = Token::new(TokenType::Asterisk, self.ch),
            '<' => tok = Token::new(TokenType::Lt, self.ch),
            '>' => tok = Token::new(TokenType::Gt, self.ch),
            '{' => tok = Token::new(TokenType::Lbrace, self.ch),
            '}' => tok = Token::new(TokenType::Rbrace, self.ch),
            NIL => {
                tok = Token {
                    tk_type: TokenType::Eof,
                    literal: String::new(),
                }
            }
            _ => {
                if self.ch.is_ascii_alphabetic() {
                    tok = Token {
                        tk_type: TokenType::Ident,
                        literal: self.read_identifier(),
                    };
                    tok.tk_type = Token::lookup_ident(&tok.literal);
                    return tok; // need not read_char, show return now
                } else if self.ch.is_ascii_digit() {
                    tok = Token {
                        tk_type: TokenType::Int,
                        literal: self.read_number(),
                    };
                    return tok; // need not read_char, show return now
                } else {
                    tok = Token::new(TokenType::Illegal, self.ch)
                }
            }
        }
        self.read_char();
        tok
    }

    fn read_identifier(&mut self) -> String {
        let position = self.position;
        while self.ch.is_ascii_alphabetic() {
            self.read_char();
        }
        String::from(&self.input[position..self.position])
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

    fn read_number(&mut self) -> String {
        let position = self.position;
        while self.ch.is_ascii_digit() {
            self.read_char();
        }
        String::from(&self.input[position..self.position])
    }

    fn peek_char(&mut self) -> char {
        if self.read_position >= self.input.len() {
            NIL
        } else {
            self.input.chars().nth(self.read_position).unwrap()
        }
    }
}
