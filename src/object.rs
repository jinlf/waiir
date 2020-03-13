use super::ast::*;
use super::environment::*;
use std::any::Any;
use std::cell::*;
use std::fmt::*;
use std::rc::*;

#[derive(PartialEq)]
pub enum ObjectType {
    IntegerObj,
    BooleanObj,
    NullObj,
    ReturnValueObj,
    ErrorObj,
    FunctionObj,
}
impl Display for ObjectType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            ObjectType::IntegerObj => write!(f, "INTEGER"),
            ObjectType::BooleanObj => write!(f, "BOOLEAN"),
            ObjectType::NullObj => write!(f, "NULL"),
            ObjectType::ReturnValueObj => write!(f, "RETURN_VALUE"),
            ObjectType::ErrorObj => write!(f, "ERROR"),
            ObjectType::FunctionObj => write!(f, "FUNCTION"),
        }
    }
}

pub trait AsAny {
    fn as_any(&self) -> &dyn Any;
}

pub trait Object: Debug + Any + AsAny {
    fn get_type(&self) -> ObjectType;
    fn inspect(&self) -> String;
    fn duplicate(&self) -> Box<dyn Object>;
}
impl<T: Object> AsAny for T {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Debug)]
pub struct Integer {
    pub value: i64,
}
impl Object for Integer {
    fn get_type(&self) -> ObjectType {
        ObjectType::IntegerObj
    }
    fn inspect(&self) -> String {
        format!("{}", self.value)
    }
    fn duplicate(&self) -> Box<dyn Object> {
        Box::new(Integer { value: self.value })
    }
}

#[derive(Debug)]
pub struct Boolean {
    pub value: bool,
}
impl Object for Boolean {
    fn get_type(&self) -> ObjectType {
        ObjectType::BooleanObj
    }
    fn inspect(&self) -> String {
        format!("{}", self.value)
    }
    fn duplicate(&self) -> Box<dyn Object> {
        Box::new(Boolean { value: self.value })
    }
}

#[derive(Debug)]
pub struct Null {}
impl Object for Null {
    fn get_type(&self) -> ObjectType {
        ObjectType::NullObj
    }
    fn inspect(&self) -> String {
        String::from("null")
    }
    fn duplicate(&self) -> Box<dyn Object> {
        Box::new(Null {})
    }
}

#[derive(Debug)]
pub struct ReturnValue {
    pub value: Box<dyn Object>,
}
impl Object for ReturnValue {
    fn get_type(&self) -> ObjectType {
        ObjectType::ReturnValueObj
    }
    fn inspect(&self) -> String {
        self.value.inspect()
    }
    fn duplicate(&self) -> Box<dyn Object> {
        Box::new(ReturnValue {
            value: self.value.duplicate(),
        })
    }
}

#[derive(Debug)]
pub struct Error {
    pub message: String,
}
impl Object for Error {
    fn get_type(&self) -> ObjectType {
        ObjectType::ErrorObj
    }
    fn inspect(&self) -> String {
        format!("ERROR: {}", self.message)
    }
    fn duplicate(&self) -> Box<dyn Object> {
        Box::new(Error {
            message: self.message.clone(),
        })
    }
}

#[derive(Debug)]
pub struct Function {
    pub function_literal: Rc<RefCell<FunctionLiteral>>,
    pub env: Rc<RefCell<Environment>>,
}
impl Object for Function {
    fn get_type(&self) -> ObjectType {
        ObjectType::FunctionObj
    }
    fn inspect(&self) -> String {
        let mut out = String::new();
        let mut params: Vec<String> = Vec::new();
        for p in self.function_literal.borrow().parameters.iter() {
            params.push(p.string());
        }
        out.push_str("fn");
        out.push_str("(");
        out.push_str(&params.join(", "));
        out.push_str(") {\n");
        out.push_str(&self.function_literal.borrow().body.string());
        out.push_str("\n}");
        out
    }
    fn duplicate(&self) -> Box<dyn Object> {
        Box::new(Function {
            function_literal: Rc::clone(&self.function_literal),
            env: Rc::clone(&self.env),
        })
    }
}
