use super::lexer::Token;
use std::any::Any;
use std::fmt::Debug;

pub trait Node: Debug {
    fn token_literal(&self) -> &str;
    fn string(&self) -> String;
    fn as_any(&self) -> &dyn Any;
}

pub trait Statement: Node {
    fn statement_node(&self);
}

pub trait Expression: Node {
    fn expression_node(&self);
}

#[derive(Debug)]
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
    fn string(&self) -> String {
        let mut out = String::new();
        for s in self.statements.iter() {
            out.push_str(&s.string());
        }
        out
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Debug)]
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
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Debug)]
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
    fn string(&self) -> String {
        let mut out = String::new();
        out.push_str(&self.value);
        out
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Debug)]
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
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Debug)]
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
    fn string(&self) -> String {
        //TODO
        String::new()
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Debug)]
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
    fn string(&self) -> String {
        self.token.literal.clone()
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Debug)]
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
    fn string(&self) -> String {
        let mut out = String::new();
        out.push_str("(");
        out.push_str(&self.operator);
        out.push_str(&self.right.as_ref().unwrap().string());
        out.push_str(")");
        out
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Debug)]
pub struct InfixExpression {
    pub token: Token,
    pub left: Option<Box<dyn Expression>>,
    pub operator: String,
    pub right: Option<Box<dyn Expression>>,
}
impl Expression for InfixExpression {
    fn expression_node(&self) {}
}
impl Node for InfixExpression {
    fn token_literal(&self) -> &str {
        &self.token.literal
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn string(&self) -> String {
        let mut out = String::new();
        out.push_str("(");
        out.push_str(&self.left.unwrap().string());
        out.push_str(&self.right.unwrap().string());
        out.push_str(")");
        out
    }
}
