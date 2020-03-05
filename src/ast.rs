use super::lexer::Token;
use std::fmt::*;

pub trait Node: std::fmt::Display {
    fn token_literal(&self) -> &str;
    fn as_any(&self) -> &dyn std::any::Any;
}

pub trait Statement: Node {
    fn statement_node(&self);
}

pub trait Expression: Node {
    fn expression_node(&self);
}

pub struct Program {
    pub statements: Vec<Box<dyn Statement>>,
}
impl Node for Program {
    fn token_literal(&self) -> &str {
        if self.statements.len() > 0 {
            self.statements[0].token_literal()
        } else {
            ""
        }
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
impl Display for Program {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Program")
    }
}

pub struct LetStatement {
    pub token: Token,
    pub name: Option<Identifier>,
    pub value: Option<Box<dyn Expression>>,
}
impl Node for LetStatement {
    fn token_literal(&self) -> &str {
        &self.token.literal[..]
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
impl Display for LetStatement {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "LetStatement")
    }
}
impl Statement for LetStatement {
    fn statement_node(&self) {}
}

pub struct Identifier {
    pub token: Token,
    pub value: String,
}
impl Expression for Identifier {
    fn expression_node(&self) {}
}
impl Node for Identifier {
    fn token_literal(&self) -> &str {
        &self.token.literal[..]
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
impl Display for Identifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Identifier")
    }
}
