use super::ast::*;
use super::object::*;

pub const TRUE: super::object::Boolean = super::object::Boolean { value: true };
pub const FALSE: super::object::Boolean = super::object::Boolean { value: false };
pub const NULL: super::object::Null = super::object::Null {};

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
    let boolean = any_node.downcast_ref::<super::ast::Boolean>();
    if boolean.is_some() {
        return native_bool_to_boolean_object(boolean.unwrap().value);
    }
    let prefix_exp = any_node.downcast_ref::<PrefixExpression>();
    if prefix_exp.is_some() {
        let right = eval(prefix_exp.unwrap().right.as_node());
        if right.is_some() {
            return eval_prefix_expression(&prefix_exp.unwrap().operator, right.unwrap());
        }
    }
    let infix_exp = any_node.downcast_ref::<InfixExpression>();
    if infix_exp.is_some() {
        let left = eval(infix_exp.unwrap().left.as_node());
        if left.is_some() {
            let right = eval(infix_exp.unwrap().right.as_node());
            if right.is_some() {
                return eval_infix_expression(
                    &infix_exp.unwrap().operator,
                    left.unwrap(),
                    right.unwrap(),
                );
            }
        }
    }
    let block_stmt = any_node.downcast_ref::<BlockStatement>();
    if block_stmt.is_some() {
        return eval_statements(&block_stmt.unwrap().statements);
    }
    let if_exp = any_node.downcast_ref::<IfExpression>();
    if if_exp.is_some() {
        return eval_if_expression(if_exp.unwrap());
    }
    let return_stmt = any_node.downcast_ref::<ReturnStatement>();
    if return_stmt.is_some() {
        let val = eval(return_stmt.unwrap().return_value.as_node());
        if val.is_some() {
            return Some(Box::new(ReturnValue {
                value: val.unwrap(),
            }));
        }
    }
    None
}

fn eval_statements(stmts: &Vec<Box<dyn Statement>>) -> Option<Box<dyn Object>> {
    let mut result: Option<Box<dyn Object>> = None;
    for statement in stmts.iter() {
        result = eval(statement.as_node());

        if result.is_some() {
            match result
                .as_ref()
                .unwrap()
                .as_any()
                .downcast_ref::<ReturnValue>()
            {
                Some(return_value) => {
                    return Some(return_value.value.duplicate());
                }
                _ => {}
            }
        }
    }
    result
}

fn eval_prefix_expression(operator: &str, right: Box<dyn Object>) -> Option<Box<dyn Object>> {
    match operator {
        "!" => eval_bang_operator_expression(right),
        "-" => eval_minus_prefix_operator_expression(right),
        _ => Some(Box::new(NULL)),
    }
}

fn eval_bang_operator_expression(right: Box<dyn Object>) -> Option<Box<dyn Object>> {
    let v = right.as_any().downcast_ref::<super::object::Boolean>();
    match v {
        Some(t) => native_bool_to_boolean_object(!t.value),
        _ => {
            let v = right.as_any().downcast_ref::<Null>();
            match v {
                Some(_) => native_bool_to_boolean_object(true),
                _ => native_bool_to_boolean_object(false),
            }
        }
    }
}

fn native_bool_to_boolean_object(val: bool) -> Option<Box<dyn Object>> {
    if val {
        return Some(Box::new(TRUE));
    }
    return Some(Box::new(FALSE));
}

fn eval_minus_prefix_operator_expression(right: Box<dyn Object>) -> Option<Box<dyn Object>> {
    if right.get_type() != ObjectType::IntegerObj {
        return None;
    }

    let value = right.as_any().downcast_ref::<Integer>().unwrap().value;
    Some(Box::new(Integer { value: -value }))
}

fn eval_infix_expression(
    operator: &str,
    left: Box<dyn Object>,
    right: Box<dyn Object>,
) -> Option<Box<dyn Object>> {
    if left.get_type() == ObjectType::IntegerObj && right.get_type() == ObjectType::IntegerObj {
        return eval_integer_infix_expression(operator, left, right);
    }
    if left.get_type() == ObjectType::BooleanObj && right.get_type() == ObjectType::BooleanObj {
        return eval_boolean_infix_expression(operator, left, right);
    }
    Some(Box::new(NULL))
}

fn eval_integer_infix_expression(
    operator: &str,
    left: Box<dyn Object>,
    right: Box<dyn Object>,
) -> Option<Box<dyn Object>> {
    let left_val = left.as_any().downcast_ref::<Integer>().unwrap().value;
    let right_val = right.as_any().downcast_ref::<Integer>().unwrap().value;
    match operator {
        "+" => Some(Box::new(Integer {
            value: left_val + right_val,
        })),
        "-" => Some(Box::new(Integer {
            value: left_val - right_val,
        })),
        "*" => Some(Box::new(Integer {
            value: left_val * right_val,
        })),
        "/" => Some(Box::new(Integer {
            value: left_val / right_val,
        })),
        "<" => native_bool_to_boolean_object(left_val < right_val),
        ">" => native_bool_to_boolean_object(left_val > right_val),
        "==" => native_bool_to_boolean_object(left_val == right_val),
        "!=" => native_bool_to_boolean_object(left_val != right_val),
        _ => Some(Box::new(NULL)),
    }
}

fn eval_boolean_infix_expression(
    operator: &str,
    left: Box<dyn Object>,
    right: Box<dyn Object>,
) -> Option<Box<dyn Object>> {
    let left_val = left
        .as_any()
        .downcast_ref::<super::object::Boolean>()
        .unwrap()
        .value;
    let right_val = right
        .as_any()
        .downcast_ref::<super::object::Boolean>()
        .unwrap()
        .value;
    match operator {
        "==" => native_bool_to_boolean_object(left_val == right_val),
        "!=" => native_bool_to_boolean_object(left_val != right_val),
        _ => None,
    }
}

fn eval_if_expression(ie: &IfExpression) -> Option<Box<dyn Object>> {
    let condition = eval(ie.condition.as_node());
    if is_truthy(condition) {
        return eval(ie.consequence.as_node());
    } else if ie.alternative.is_some() {
        return eval(ie.alternative.as_ref().unwrap().as_node());
    } else {
        return Some(Box::new(NULL));
    }
}

fn is_truthy(obj: Option<Box<dyn Object>>) -> bool {
    // TODO what about other value
    match obj {
        Some(v) => {
            let null = v.as_any().downcast_ref::<Null>();
            if null.is_some() {
                return false;
            }
            let bo = v.as_any().downcast_ref::<super::object::Boolean>();
            if bo.is_some() {
                return bo.unwrap().value;
            }
            return true;
        }
        _ => false,
    }
}
