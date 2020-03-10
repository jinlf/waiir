use std::any::Any;
use std::fmt::*;

#[derive(PartialEq)]
pub enum ObjectType {
    IntegerObj,
    BooleanObj,
    NullObj,
    ReturnValueObj,
    ErrorObj,
}
impl Display for ObjectType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            ObjectType::IntegerObj => write!(f, "INTEGER"),
            ObjectType::BooleanObj => write!(f, "BOOLEAN"),
            ObjectType::NullObj => write!(f, "NULL"),
            ObjectType::ReturnValueObj => write!(f, "RETURN_VALUE"),
            ObjectType::ErrorObj => write!(f, "ERROR"),
        }
    }
}

pub trait Object: Debug {
    fn get_type(&self) -> ObjectType;
    fn inspect(&self) -> String;
    fn as_any(&self) -> &dyn Any;
    fn duplicate(&self) -> Box<dyn Object>;
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
    fn as_any(&self) -> &dyn Any {
        self
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
    fn as_any(&self) -> &dyn Any {
        self
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
    fn as_any(&self) -> &dyn Any {
        self
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
    fn as_any(&self) -> &dyn Any {
        self
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
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn duplicate(&self) -> Box<dyn Object> {
        Box::new(Error {
            message: self.message.clone(),
        })
    }
}
