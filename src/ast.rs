use super::lexer::Token;

trait Node {
    fn token_literal(&mut self) -> &str;
}

trait Statement : Node {
    fn statement_node(&mut self);
}

trait Expression : Node {
    fn expression_node(&mut self);
}

pub struct Program {
    statments: Vec<Box<dyn Statement>>,
}
impl Node for Program {
    fn token_literal(&mut self) -> &str {
        if self.statments.len() > 0 {
            self.statments[0].token_literal()
        } else {
            ""
        }
    }
}

struct LetStatement<'a> {
    token: Token,
    name: &'a Identifier,
    value: dyn Expression,
}
impl<'a> Node for LetStatement<'a> {
    fn token_literal(&mut self) -> &str {
        &self.token.literal[..]
    }
}
impl<'a> Statement for LetStatement<'a> {
    fn statement_node(&mut self) {}
}

struct Identifier {
    token: Token,
    value: String
}
impl Expression for Identifier {
    fn expression_node(&mut self) {

    }   
}
impl Node for Identifier {
    fn token_literal(&mut self) -> &str {
        &self.token.literal[..]
    }   
}

pub fn new_program_ast_node() -> Program {
    Program{
        statments:Vec::new(),
    }
}