use super::lexer::Token;
use std::any::Any;
use std::fmt::{Display, Formatter, Result};

pub trait Node: Display {
    fn token_literal(&self) -> &str;
    fn as_any(&self) -> &dyn Any;
    fn string(&self) -> String;
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
        let statements = &self.statements;
        if statements.len() > 0 {
            statements[0].token_literal()
        } else {
            ""
        }
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn string(&self) -> String {
        let mut out = String::new();
        for s in self.statements.iter() {
            out.push_str(&s.string());
        }
        out
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
impl Statement for LetStatement {
    fn statement_node(&self) {}
}
impl Node for LetStatement {
    fn token_literal(&self) -> &str {
        &self.token.literal[..]
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn string(&self) -> String {
        let mut out = String::new();
        out.push_str(self.token_literal());
        out.push_str(" ");
        out.push_str(&self.name.as_ref().unwrap().string());
        out.push_str(" = ");
        out.push_str(&self.value.as_ref().unwrap().string());
        out.push_str(";");
        out
    }
}
impl Display for LetStatement {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "LetStatement")
    }
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
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn string(&self) -> String {
        let mut out = String::new();
        out.push_str(&self.value);
        out
    }
}
impl Display for Identifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Identifier")
    }
}

pub struct ReturnStatement {
    pub token: Token,
    pub return_value: Option<Box<dyn Expression>>,
}
impl Statement for ReturnStatement {
    fn statement_node(&self) {}
}
impl Node for ReturnStatement {
    fn token_literal(&self) -> &str {
        &self.token.literal
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn string(&self) -> String {
        let mut out = String::new();
        out.push_str(self.token_literal());
        out.push_str(" ");
        match &self.return_value {
            Some(return_value) => {
                out.push_str(&return_value.string());
            }
            _ => {}
        }
        out.push_str(";");
        out
    }
}
impl Display for ReturnStatement {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "ReturnStatement")
    }
}

pub struct ExpressionStmt {
    pub token: Token,
    pub expression: Option<Box<dyn Expression>>,
}
impl Statement for ExpressionStmt {
    fn statement_node(&self) {}
}
impl Node for ExpressionStmt {
    fn token_literal(&self) -> &str {
        &self.token.literal
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn string(&self) -> String {
        //TODO
        String::new()
    }
}
impl Display for ExpressionStmt {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "ExpressionStmt")
    }
}

pub struct IntegerLiteral {
    pub token: Token,
    pub value: i64,
}
impl Expression for IntegerLiteral {
    fn expression_node(&self) {}
}
impl Node for IntegerLiteral {
    fn token_literal(&self) -> &str {
        &self.token.literal
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn string(&self) -> String {
        self.token.literal.clone()
    }
}
impl Display for IntegerLiteral {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "IntegerLiteral")
    }
}

pub struct PrefixExpression {
    pub token: Token,
    pub operator: String,
    pub right: Option<Box<dyn Expression>>,
}
impl Expression for PrefixExpression {
    fn expression_node(&self) {}
}
impl Node for PrefixExpression {
    fn token_literal(&self) -> &str {
        &self.token.literal
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn string(&self) -> String {
        let mut out = String::new();
        out.push_str("(");
        out.push_str(&self.operator);
        out.push_str(&self.right.as_ref().unwrap().string());
        out.push_str(")");
        out
    }
}
impl Display for PrefixExpression {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "PrefixExpression")
    }
}
