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

#[derive(Debug, Clone)]
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
        out.push_str(&self.return_value.as_ref().unwrap().string());
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
        match &self.expression {
            Some(exp) => exp.string(),
            _ => String::new(),
        }
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
    fn string(&self) -> String {
        let mut out = String::new();
        out.push_str("(");
        out.push_str(&self.left.as_ref().unwrap().string());
        out.push_str(" ");
        out.push_str(&self.operator);
        out.push_str(" ");
        out.push_str(&self.right.as_ref().unwrap().string());
        out.push_str(")");
        out
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Debug)]
pub struct Boolean {
    pub token: Token,
    pub value: bool,
}
impl Expression for Boolean {
    fn expression_node(&self) {}
}
impl Node for Boolean {
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
pub struct IfExpression {
    pub token: Token,
    pub condition: Option<Box<dyn Expression>>,
    pub consequence: Option<BlockStatement>,
    pub alternative: Option<BlockStatement>,
}
impl Expression for IfExpression {
    fn expression_node(&self) {}
}
impl Node for IfExpression {
    fn token_literal(&self) -> &str {
        &self.token.literal
    }
    fn string(&self) -> String {
        let mut out = String::new();
        out.push_str("if");
        out.push_str(&self.condition.as_ref().unwrap().string());
        out.push_str(" ");
        out.push_str(&self.consequence.as_ref().unwrap().string());
        match &self.alternative {
            Some(alternative) => {
                out.push_str(" else ");
                out.push_str(&alternative.string());
            }
            _ => {}
        }
        out
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Debug)]
pub struct BlockStatement {
    pub token: Token,
    pub statements: Vec<Box<dyn Statement>>,
}
impl Statement for BlockStatement {
    fn statement_node(&self) {}
}
impl Node for BlockStatement {
    fn token_literal(&self) -> &str {
        &self.token.literal
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
pub struct FunctionLiteral {
    pub token: Token,
    pub parameters: Vec<Identifier>,
    pub body: Option<BlockStatement>,
}
impl Expression for FunctionLiteral {
    fn expression_node(&self) {}
}
impl Node for FunctionLiteral {
    fn token_literal(&self) -> &str {
        &self.token.literal
    }
    fn string(&self) -> String {
        let mut out = String::new();
        let mut params: Vec<String> = Vec::new();
        for p in self.parameters.iter() {
            params.push(p.string());
        }
        out.push_str(self.token_literal());
        out.push_str("(");
        out.push_str(&params.join(", "));
        out.push_str(")");
        out.push_str(&self.body.as_ref().unwrap().string());
        out
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Debug)]
pub struct CallExpression {
    pub token: Token,
    pub function: Option<Box<dyn Expression>>,
    pub arguments: Vec<Box<dyn Expression>>,
}
impl Expression for CallExpression {
    fn expression_node(&self) {}
}
impl Node for CallExpression {
    fn token_literal(&self) -> &str {
        &self.token.literal
    }
    fn string(&self) -> String {
        let mut out = String::new();
        let mut args: Vec<String> = Vec::new();
        for a in self.arguments.iter() {
            args.push(a.string());
        }
        out.push_str(&self.function.as_ref().unwrap().string());
        out.push_str("(");
        out.push_str(&args.join(", "));
        out.push_str(")");
        out
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}
