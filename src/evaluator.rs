use super::ast::*;
use super::environment::*;
use super::object::*;
use std::any::*;
use std::cell::*;
use std::ops::*;
use std::rc::*;

pub const TRUE: super::object::Boolean = super::object::Boolean { value: true };
pub const FALSE: super::object::Boolean = super::object::Boolean { value: false };
pub const NULL: super::object::Null = super::object::Null {};

fn eval_statement(stmt: &dyn Statement, env: &Rc<RefCell<Environment>>) -> Option<Box<dyn Object>> {
    println!("eval_statement: {:?}", stmt.string());
    if let Some(expression_stmt) = stmt.as_any().downcast_ref::<ExpressionStmt>() {
        return eval_expression(&*expression_stmt.expression, env);
    }
    if let Some(block_stmt) = stmt.as_any().downcast_ref::<BlockStatement>() {
        return eval_block_statement(block_stmt, env);
    }
    if let Some(return_stmt) = stmt.as_any().downcast_ref::<ReturnStatement>() {
        if let Some(val) = eval_expression(&*return_stmt.return_value, env) {
            if is_error(&val) {
                return Some(val);
            }
            return Some(Box::new(ReturnValue { value: val }));
        }
    }
    if let Some(let_stmt) = stmt.as_any().downcast_ref::<LetStatement>() {
        if let Some(val) = eval_expression(&*let_stmt.value, env) {
            if is_error(&val) {
                return Some(val);
            }
            env.borrow_mut().set(let_stmt.name.value.clone(), val);
        }
    }
    None
}

fn eval_expression(
    exp: &dyn Expression,
    env: &Rc<RefCell<Environment>>,
) -> Option<Box<dyn Object>> {
    println!("eval_expression: {:?}", exp.string());
    if let Some(integer_literal) = exp.as_any().downcast_ref::<IntegerLiteral>() {
        return Some(Box::new(Integer {
            value: integer_literal.value,
        }));
    }
    if let Some(boolean) = exp.as_any().downcast_ref::<super::ast::Boolean>() {
        return native_bool_to_boolean_object(boolean.value);
    }
    if let Some(prefix_exp) = exp.as_any().downcast_ref::<PrefixExpression>() {
        if let Some(right) = eval_expression(&*prefix_exp.right, env) {
            if is_error(&right) {
                return Some(right);
            }
            return eval_prefix_expression(&prefix_exp.operator, right);
        }
    }
    if let Some(infix_exp) = exp.as_any().downcast_ref::<InfixExpression>() {
        if let Some(left) = eval_expression(&*infix_exp.left, env) {
            if is_error(&left) {
                return Some(left);
            }
            if let Some(right) = eval_expression(&*infix_exp.right, env) {
                if is_error(&right) {
                    return Some(right);
                }
                return eval_infix_expression(&infix_exp.operator, left, right);
            }
        }
    }
    if let Some(if_exp) = exp.as_any().downcast_ref::<IfExpression>() {
        return eval_if_expression(&*if_exp, env);
    }
    if let Some(ident) = exp.as_any().downcast_ref::<Identifier>() {
        return eval_identifier(&*ident, env);
    }
    if let Some(function_literal) = exp.as_any().downcast_ref::<FunctionLiteral>() {
        return Some(Box::new(Function {
            function_literal: Rc::new(RefCell::new(function_literal)),
            env: Rc::clone(env),
        }));
    }
    if let Some(call_exp) = exp.as_any().downcast_ref::<CallExpression>() {
        if let Some(function) = eval_expression(&*call_exp.function, env) {
            if is_error(&function) {
                return Some(function);
            }

            let args = eval_expressions(&call_exp.arguments, env);
            if args.len() == 1 && args[0].is_some() && is_error(&args[0].as_ref().unwrap()) {
                return Some(args[0].as_ref().unwrap().duplicate());
            }

            return apply_function(function, args);
        }
    }
    None
}
pub fn eval(node: &dyn Node, env: &Rc<RefCell<Environment>>) -> Option<Box<dyn Object>> {
    println!("eval: {:?}", node.string());
    if let Some(program) = node.as_any().downcast_ref::<Program>() {
        return eval_program(program, env);
    }
    None
}

fn apply_function(
    func: Box<dyn Object>,
    args: Vec<Option<Box<dyn Object>>>,
) -> Option<Box<dyn Object>> {
    let function = func.as_any().downcast_ref::<Function>();
    if function.is_none() {
        return Some(Box::new(new_error(format_args!(
            "not a function: {}",
            func.get_type()
        ))));
    }

    let extended_env = Rc::new(RefCell::new(extend_function_env(function.unwrap(), args)));
    let evaluated = eval_statement(
        &function.unwrap().function_literal.borrow().body,
        &extended_env,
    );
    return unwrap_return_value(evaluated);
}

fn unwrap_return_value(obj: Option<Box<dyn Object>>) -> Option<Box<dyn Object>> {
    if obj.is_none() {
        return obj;
    }
    match obj.as_ref().unwrap().as_any().downcast_ref::<ReturnValue>() {
        Some(return_value) => Some(return_value.value.duplicate()),
        _ => obj,
    }
}

fn extend_function_env(func: &Function, args: Vec<Option<Box<dyn Object>>>) -> Environment {
    let mut env = new_enclosed_environment(&func.env);
    for (param_idx, param) in func.function_literal.borrow().parameters.iter().enumerate() {
        env.set(
            param.value.clone(),
            args[param_idx].as_ref().unwrap().duplicate(),
        );
    }
    env
}

fn eval_expressions(
    exps: &Vec<Box<dyn Expression>>,
    env: &Rc<RefCell<Environment>>,
) -> Vec<Option<Box<dyn Object>>> {
    println!("eval_expressions:");
    for ee in exps.iter() {
        println!("\t{:?}", ee);
    }
    let mut result: Vec<Option<Box<dyn Object>>> = Vec::new();
    for e in exps.iter() {
        let evaluated = eval_expression(e.as_ref(), env);
        if evaluated.is_some() && is_error(&evaluated.as_ref().unwrap()) {
            return vec![Some(evaluated.unwrap().duplicate())];
        }
        result.push(evaluated);
    }
    result
}

fn is_error(node: &Box<dyn Object>) -> bool {
    node.get_type() == ObjectType::ErrorObj
}

fn eval_program(program: &Program, env: &Rc<RefCell<Environment>>) -> Option<Box<dyn Object>> {
    println!("eval_program: {:?}", program.string());
    let mut result: Option<Box<dyn Object>> = None;
    for statement in program.statements.iter() {
        result = eval_statement(statement.as_ref(), env);

        if result.is_some() {
            let return_value = result
                .as_ref()
                .unwrap()
                .as_any()
                .downcast_ref::<ReturnValue>();
            if return_value.is_some() {
                return Some(return_value.unwrap().value.duplicate());
            }

            let error = result
                .as_ref()
                .unwrap()
                .as_any()
                .downcast_ref::<super::object::Error>();
            if error.is_some() {
                return Some(error.unwrap().duplicate());
            }
        }
    }
    result
}

fn eval_block_statement(
    block: &BlockStatement,
    env: &Rc<RefCell<Environment>>,
) -> Option<Box<dyn Object>> {
    println!("eval_block_statement: {:?}", block.string());
    let mut result: Option<Box<dyn Object>> = None;
    for statement in block.statements.iter() {
        result = eval_statement(statement.as_ref(), env);
        if result.is_some() {
            let rt = result.as_ref().unwrap().get_type();
            if rt == ObjectType::ReturnValueObj || rt == ObjectType::ErrorObj {
                return Some(result.unwrap().duplicate());
            }
        }
    }
    result
}

fn eval_prefix_expression(operator: &str, right: Box<dyn Object>) -> Option<Box<dyn Object>> {
    println!("eval_prefix_expression: {} {:?}", operator, right);
    match operator {
        "!" => eval_bang_operator_expression(right),
        "-" => eval_minus_prefix_operator_expression(right),
        _ => Some(Box::new(new_error(format_args!(
            "unknown operator: {}{}",
            operator,
            right.get_type()
        )))),
    }
}

fn eval_bang_operator_expression(right: Box<dyn Object>) -> Option<Box<dyn Object>> {
    println!("eval_bang_operator_expression: {:?}", right);
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
    println!("eval_minus_prefix_operator_expression: {:?}", right);
    if right.get_type() != ObjectType::IntegerObj {
        return Some(Box::new(new_error(format_args!(
            "unknown operator: -{}",
            right.get_type()
        ))));
    }

    let value = right.as_any().downcast_ref::<Integer>().unwrap().value;
    Some(Box::new(Integer { value: -value }))
}

fn eval_infix_expression(
    operator: &str,
    left: Box<dyn Object>,
    right: Box<dyn Object>,
) -> Option<Box<dyn Object>> {
    println!("eval_infix_expression: {} {:?} {:?}", operator, left, right);
    if left.get_type() == ObjectType::IntegerObj && right.get_type() == ObjectType::IntegerObj {
        return eval_integer_infix_expression(operator, left, right);
    }
    if left.get_type() == ObjectType::BooleanObj && right.get_type() == ObjectType::BooleanObj {
        return eval_boolean_infix_expression(operator, left, right);
    }
    if left.get_type() != right.get_type() {
        return Some(Box::new(new_error(format_args!(
            "type mismatch: {} {} {}",
            left.get_type(),
            operator,
            right.get_type()
        ))));
    }
    Some(Box::new(new_error(format_args!(
        "unknown operator: {} {} {}",
        left.get_type(),
        operator,
        right.get_type()
    ))))
}

fn eval_integer_infix_expression(
    operator: &str,
    left: Box<dyn Object>,
    right: Box<dyn Object>,
) -> Option<Box<dyn Object>> {
    println!(
        "eval_integer_infix_expression: {} {:?} {:?}",
        operator, left, right
    );
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
        _ => Some(Box::new(new_error(format_args!(
            "unknown operator: {} {} {}",
            left.get_type(),
            operator,
            right.get_type()
        )))),
    }
}

fn eval_boolean_infix_expression(
    operator: &str,
    left: Box<dyn Object>,
    right: Box<dyn Object>,
) -> Option<Box<dyn Object>> {
    println!(
        "eval_boolean_infix_expression: {} {:?} {:?}",
        operator, left, right
    );
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
        _ => Some(Box::new(new_error(format_args!(
            "unknown operator: {} {} {}",
            left.get_type(),
            operator,
            right.get_type()
        )))),
    }
}

fn eval_if_expression(
    ie: &IfExpression,
    env: &Rc<RefCell<Environment>>,
) -> Option<Box<dyn Object>> {
    println!("eval_if_expression: {:?}", ie.string());
    let condition = eval_expression(ie.condition.as_ref(), env);
    if condition.is_some() && is_error(&condition.as_ref().unwrap()) {
        return Some(condition.unwrap());
    }
    if is_truthy(condition) {
        return eval_statement(&ie.consequence, env);
    } else if ie.alternative.is_some() {
        return eval_statement(ie.alternative.as_ref().unwrap(), env);
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

fn new_error(args: std::fmt::Arguments<'_>) -> super::object::Error {
    super::object::Error {
        message: std::fmt::format(args),
    }
}

fn eval_identifier(node: &Identifier, env: &Rc<RefCell<Environment>>) -> Option<Box<dyn Object>> {
    println!("eval_identifier: {:?}", node.string());
    match env.borrow().get(&node.value) {
        Some(val) => Some(val.duplicate()),
        _ => Some(Box::new(new_error(format_args!(
            "identifier not found: {}",
            node.value
        )))),
    }
}
