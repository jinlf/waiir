use std::str;
use std::str::FromStr;

#[derive(Debug, PartialEq, Copy, Clone)]
#[warn(dead_code)]
pub enum TokenType {
    ILLEGAL,
    EOF,
    // Identifiers + literals
    IDENT, // add, foobar, x, y, ...
    INT,
    // Operators
    ASSIGN,
    PLUS,
    MINUS,
    BANG,
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

pub struct Lexer {
    input: String,
    position: usize,      // current position in input (points to current char)
    read_position: usize, // current reading position in input (after current char)
    pub ch: char,         // current char under examination
}

const NIL: char = 0 as char;

impl Lexer {
    pub fn new(input: &str) -> Lexer {
        let mut l: Lexer = Lexer {
            input: String::from_str(input).unwrap(),
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
        String::from_str(&self.input[position..self.position]).unwrap()
    }

    fn read_number(&mut self) -> String {
        let position = self.position;
        while self.ch.is_ascii_digit() {
            self.read_char();
        }
        String::from_str(&self.input[position..self.position]).unwrap()
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_next_token1() {
        let input = "=+(){},;";
        let tests = [
            (TokenType::ASSIGN, "="),
            (TokenType::PLUS, "+"),
            (TokenType::LPAREN, "("),
            (TokenType::RPAREN, ")"),
            (TokenType::LBRACE, "{"),
            (TokenType::RBRACE, "}"),
            (TokenType::COMMA, ","),
            (TokenType::SEMICOLON, ";"),
            (TokenType::EOF, ""),
        ];
        let mut l = Lexer::new(input);
        for (index, value) in tests.iter().enumerate() {
            let tok = l.next_token();
            assert!(
                tok.tk_type == value.0,
                "tests[{}] - tokentype wrong. expected={:?}, got={:?}",
                index,
                value.0,
                tok.tk_type
            );
            assert!(
                tok.literal == value.1,
                "test[{} - literal wrong. expected={}, got={}",
                index,
                value.1,
                tok.literal
            );
        }
    }
    #[test]
    fn test_next_token2() {
        let input = "let five = 5;
        let ten = 10;
        let add = fn(x, y) { x + y;
        };
        let result = add(five, ten);";
        let tests = [
            (TokenType::LET, "let"),
            (TokenType::IDENT, "five"),
            (TokenType::ASSIGN, "="),
            (TokenType::INT, "5"),
            (TokenType::SEMICOLON, ";"),
            (TokenType::LET, "let"),
            (TokenType::IDENT, "ten"),
            (TokenType::ASSIGN, "="),
            (TokenType::INT, "10"),
            (TokenType::SEMICOLON, ";"),
            (TokenType::LET, "let"),
            (TokenType::IDENT, "add"),
            (TokenType::ASSIGN, "="),
            (TokenType::FUNCTION, "fn"),
            (TokenType::LPAREN, "("),
            (TokenType::IDENT, "x"),
            (TokenType::COMMA, ","),
            (TokenType::IDENT, "y"),
            (TokenType::RPAREN, ")"),
            (TokenType::LBRACE, "{"),
            (TokenType::IDENT, "x"),
            (TokenType::PLUS, "+"),
            (TokenType::IDENT, "y"),
            (TokenType::SEMICOLON, ";"),
            (TokenType::RBRACE, "}"),
            (TokenType::SEMICOLON, ";"),
            (TokenType::LET, "let"),
            (TokenType::IDENT, "result"),
            (TokenType::ASSIGN, "="),
            (TokenType::IDENT, "add"),
            (TokenType::LPAREN, "("),
            (TokenType::IDENT, "five"),
            (TokenType::COMMA, ","),
            (TokenType::IDENT, "ten"),
            (TokenType::RPAREN, ")"),
            (TokenType::SEMICOLON, ";"),
            (TokenType::EOF, ""),
        ];
        let mut l = Lexer::new(input);
        for (index, value) in tests.iter().enumerate() {
            let tok = l.next_token();
            assert!(
                tok.tk_type == value.0,
                "tests[{}] - tokentype wrong. expected={:?}, got={:?}",
                index,
                value.0,
                tok.tk_type
            );
            assert!(
                tok.literal == value.1,
                "test[{} - literal wrong. expected={}, got={}",
                index,
                value.1,
                tok.literal
            );
        }
    }

    #[test]
    fn test_next_token3() {
        let input = "let five = 5;
        let ten = 10;
        let add = fn(x, y) { x + y;
        };
        let result = add(five, ten); 
        !-/*5;
        5 < 10 > 5;
        
        if (5 < 10) { 
            return true;
        } else {
            return false;
        }
        10 == 10; 
        10 != 9;
        ";
        let tests = [
            (TokenType::LET, "let"),
            (TokenType::IDENT, "five"),
            (TokenType::ASSIGN, "="),
            (TokenType::INT, "5"),
            (TokenType::SEMICOLON, ";"),
            (TokenType::LET, "let"),
            (TokenType::IDENT, "ten"),
            (TokenType::ASSIGN, "="),
            (TokenType::INT, "10"),
            (TokenType::SEMICOLON, ";"),
            (TokenType::LET, "let"),
            (TokenType::IDENT, "add"),
            (TokenType::ASSIGN, "="),
            (TokenType::FUNCTION, "fn"),
            (TokenType::LPAREN, "("),
            (TokenType::IDENT, "x"),
            (TokenType::COMMA, ","),
            (TokenType::IDENT, "y"),
            (TokenType::RPAREN, ")"),
            (TokenType::LBRACE, "{"),
            (TokenType::IDENT, "x"),
            (TokenType::PLUS, "+"),
            (TokenType::IDENT, "y"),
            (TokenType::SEMICOLON, ";"),
            (TokenType::RBRACE, "}"),
            (TokenType::SEMICOLON, ";"),
            (TokenType::LET, "let"),
            (TokenType::IDENT, "result"),
            (TokenType::ASSIGN, "="),
            (TokenType::IDENT, "add"),
            (TokenType::LPAREN, "("),
            (TokenType::IDENT, "five"),
            (TokenType::COMMA, ","),
            (TokenType::IDENT, "ten"),
            (TokenType::RPAREN, ")"),
            (TokenType::SEMICOLON, ";"),
            (TokenType::BANG, "!"),
            (TokenType::MINUS, "-"),
            (TokenType::SLASH, "/"),
            (TokenType::ASTERISK, "*"),
            (TokenType::INT, "5"),
            (TokenType::SEMICOLON, ";"),
            (TokenType::INT, "5"),
            (TokenType::LT, "<"),
            (TokenType::INT, "10"),
            (TokenType::GT, ">"),
            (TokenType::INT, "5"),
            (TokenType::SEMICOLON, ";"),
            (TokenType::IF, "if"),
            (TokenType::LPAREN, "("),
            (TokenType::INT, "5"),
            (TokenType::LT, "<"),
            (TokenType::INT, "10"),
            (TokenType::RPAREN, ")"),
            (TokenType::LBRACE, "{"),
            (TokenType::RETURN, "return"),
            (TokenType::TRUE, "true"),
            (TokenType::SEMICOLON, ";"),
            (TokenType::RBRACE, "}"),
            (TokenType::ELSE, "else"),
            (TokenType::LBRACE, "{"),
            (TokenType::RETURN, "return"),
            (TokenType::FALSE, "false"),
            (TokenType::SEMICOLON, ";"),
            (TokenType::RBRACE, "}"),
            (TokenType::INT, "10"),
            (TokenType::EQ, "=="),
            (TokenType::INT, "10"),
            (TokenType::SEMICOLON, ";"),
            (TokenType::INT, "10"),
            (TokenType::NOTEQ, "!="),
            (TokenType::INT, "9"),
            (TokenType::SEMICOLON, ";"),
            (TokenType::EOF, ""),
        ];
        let mut l = Lexer::new(input);
        for (index, value) in tests.iter().enumerate() {
            let tok = l.next_token();
            assert!(
                tok.tk_type == value.0,
                "tests[{}] - tokentype wrong. expected={:?}, got={:?}",
                index,
                value.0,
                tok.tk_type
            );
            assert!(
                tok.literal == value.1,
                "test[{} - literal wrong. expected={}, got={}",
                index,
                value.1,
                tok.literal
            );
        }
    }
}
