use std::collections::HashMap;

use thiserror;

#[derive(Debug, Clone)]
pub enum Type {
    Int,
    Float,
    Bool,
    Function,
    Type,
    Class,
}

#[derive(Debug, Clone)]
pub enum TypeValue {
    Int,
    Float,
    Bool,
    Type(Type),
    Function(*const FunctionDefinition),
    Class(*const ClassDefinition),
}

impl std::fmt::Display for TypeValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TypeValue::Int => write!(f, "Int"),
            TypeValue::Float => write!(f, "Float"),
            TypeValue::Bool => write!(f, "Bool"),
            TypeValue::Function(func) => unsafe { write!(f, "Function(\"{}\")", (**func).name) },
            TypeValue::Class(class) => unsafe { write!(f, "Class(\"{}\")", (**class).name) },
            TypeValue::Type(t) => write!(f, "Type"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Value {
    Int(i64),
    Float(f64),
    Bool(bool),
    Function(*const FunctionDefinition),
    Class(Box<ClassInstance>),
    TypeValue(Box<TypeValue>),
}

#[derive(Debug, thiserror::Error)]
pub enum ValueError {
    #[error("Function {0} not found for types {1}, {2}")]
    FunctionNotAvailable(&'static str, TypeValue, TypeValue),
}

impl Value {
    pub fn get_type(&self) -> TypeValue {
        match self {
            Value::Int(_) => TypeValue::Int,
            Value::Float(_) => TypeValue::Float,
            Value::Bool(_) => TypeValue::Bool,
            Value::Function(val) => TypeValue::Function(*val),
            Value::Class(val) => TypeValue::Class(val.defintion),
            Value::TypeValue(val) => (**val).clone(),
        }
    }

    pub fn add(&self, other: &Value) -> Result<Value, ValueError> {
        match (self, other) {
            (Value::Int(x), Value::Int(y)) => Ok(Value::Int(x + y)),
            (Value::Float(x), Value::Float(y)) => Ok(Value::Float(x + y)),
            _ => Err(ValueError::FunctionNotAvailable("add", self.get_type(), other.get_type())),
        }
    }

    pub fn mul(&self, other: &Value) -> Result<Value, ValueError> {
        match (self, other) {
            (Value::Int(x), Value::Int(y)) => Ok(Value::Int(x * y)),
            (Value::Float(x), Value::Float(y)) => Ok(Value::Float(x * y)),
            _ => Err(ValueError::FunctionNotAvailable("add", self.get_type(), other.get_type())),
        }
    }
}

#[derive(Debug, Clone)]
pub struct FunctionDefinition {
    name: String,
    parameters: Vec<Type>,
    return_type: Type,
}

#[derive(Debug, Clone)]
pub struct ClassDefinition {
    name: String,
    functions: Vec<FunctionDefinition>,
}

#[derive(Debug, Clone)]
pub struct EnumDefinition {
    name: String,
}

#[derive(Debug, Clone)]
pub struct ClassInstance {
    defintion: *const ClassDefinition,
    instance_variables: HashMap<String, Value>,
}

#[derive(Debug, Clone)]
pub struct Enum {}

