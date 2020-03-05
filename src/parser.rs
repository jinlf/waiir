use super::ast::*;
use super::lexer::*;

struct Parser<'a> {
    l: &'a mut Lexer<'a>,
    cur_token: Token,
    peek_token: Token,
}

impl<'a> Parser<'a> {
    fn new(lex: &'a mut Lexer<'a>) -> Parser<'a> {
        Parser {
            l: lex,
            cur_token: Token::new(TokenType::ILLEGAL,""),
            peek_token: Token::new(TokenType::ILLEGAL,""),
        }
    }

    fn init(&mut self) {
        self.next_token();
        self.next_token();
    }

    fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.l.next_token();
    }

    fn parse_program(&mut self) -> Option<Program> {
        None
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
        p.init();
        p.parse_program().unwrap();
    }
}
