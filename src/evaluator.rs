use super::ast::*;
use super::object::*;

pub fn eval(node: &dyn Node) -> Option<Box<dyn Object>> {
    let any_node = node.as_any();
    let program = any_node.downcast_ref::<Program>();
    if program.is_some() {
        return eval_statements(&program.unwrap().statements);
    }
    let expression_stmt = any_node.downcast_ref::<ExpressionStmt>();
    if expression_stmt.is_some() {
        return eval(expression_stmt.unwrap().expression.as_node());
    }
    let integer_literal = any_node.downcast_ref::<IntegerLiteral>();
    if integer_literal.is_some() {
        return Some(Box::new(Integer {
            value: integer_literal.unwrap().value,
        }));
    }
    None
}

fn eval_statements(stmts: &Vec<Box<dyn Statement>>) -> Option<Box<dyn Object>> {
    let mut result: Option<Box<dyn Object>> = None;
    for statement in stmts.iter() {
        result = eval(statement.as_node());
    }
    result
}
