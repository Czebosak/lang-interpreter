use std::collections::HashMap;

use thiserror;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Int,
    Float,
    Bool,
    Function,
    Type,
    Class,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeValue {
    Int,
    Float,
    Bool,
    Type(Type),
    Function(*const FunctionDefinition),
    Class(*const ClassDefinition),
}

impl TypeValue {
    pub fn is_numerical(&self) -> bool {
        *self == TypeValue::Int || *self == TypeValue::Float
    }
}

impl std::fmt::Display for TypeValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TypeValue::Int => write!(f, "Int"),
            TypeValue::Float => write!(f, "Float"),
            TypeValue::Bool => write!(f, "Bool"),
            TypeValue::Function(func) => unsafe { write!(f, "Function(\"{}\")", (**func).name) },
            TypeValue::Class(class) => unsafe { write!(f, "Class(\"{}\")", (**class).name) },
            TypeValue::Type(_) => write!(f, "Type"),
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
    #[error("Division by zero")]
    DivisionByZero,
    #[error("Variable with identifier not found")]
    VariableWithIdentifierNotFound,
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

    pub fn is_numerical(&self) -> bool {
        match self {
            Value::Int(_) | Value::Float(_) => true,
            _ => false,
        }
    }

    fn numerical_op<IntF, FloatF>(&self, other: &Value, int_op: IntF, float_op: FloatF) -> Value
    where IntF: Fn(i64, i64) -> i64, FloatF: Fn(f64, f64) -> f64 {
        let extract_float = |v: &Value| -> f64 {
            match v {
                Value::Int(n) => *n as f64,
                Value::Float(n) => *n,
                _ => unreachable!(),
            }
        };

        match (self, other) {
            (Value::Int(x), Value::Int(y)) => Value::Int(int_op(*x, *y)),
            (Value::Float(x), Value::Float(y)) => Value::Float(float_op(*x, *y)),
            _ => {
                Value::Float(float_op(extract_float(self), extract_float(other)))
            }
        }
    }

    pub fn equals(&self, other: &Value) -> Result<bool, ValueError> {
        let extract_float = |v: &Value| -> f64 {
            match v {
                Value::Int(n) => *n as f64,
                Value::Float(n) => *n,
                _ => unreachable!(),
            }
        };

        match (self, other) {
            (Value::Int(x), Value::Int(y)) => Ok(*x == *y),
            (Value::Float(x), Value::Float(y)) => Ok(*x == *y),
            (Value::Bool(a), Value::Bool(b)) => Ok(*a == *b),
            _ => {
                if self.is_numerical() && other.is_numerical() {
                    Ok(extract_float(self) == extract_float(other))
                } else {
                    Err(ValueError::FunctionNotAvailable("equals", self.get_type(), other.get_type()))
                }
            }
        }
    }

    pub fn add(&self, other: &Value) -> Result<Value, ValueError> {
        fn op<T: std::ops::Add<Output = T>>(x: T, y: T) -> T {
            x + y
        }

        match (self, other) {
            (x, y) if x.is_numerical() && y.is_numerical() => {
                Ok(self.numerical_op(other, op, op))
            },
            _ => Err(ValueError::FunctionNotAvailable("add", self.get_type(), other.get_type())),
        }
    }

    pub fn sub(&self, other: &Value) -> Result<Value, ValueError> {
        fn op<T: std::ops::Sub<Output = T>>(x: T, y: T) -> T {
            x - y
        }

        match (self, other) {
            (x, y) if x.is_numerical() && y.is_numerical() => {
                Ok(self.numerical_op(other, op, op))
            },
            _ => Err(ValueError::FunctionNotAvailable("sub", self.get_type(), other.get_type())),
        }
    }

    pub fn mul(&self, other: &Value) -> Result<Value, ValueError> {
        fn op<T: std::ops::Mul<Output = T>>(x: T, y: T) -> T {
            x * y
        }

        match (self, other) {
            (x, y) if x.is_numerical() && y.is_numerical() => {
                Ok(self.numerical_op(other, op, op))
            },
            _ => Err(ValueError::FunctionNotAvailable("mul", self.get_type(), other.get_type())),
        }
    }

    pub fn div(&self, other: &Value) -> Result<Value, ValueError> {
        match (self, other) {
            (Value::Int(x), Value::Int(y)) => if *y != 0 { Ok(Value::Int(x / y)) } else { Err(ValueError::DivisionByZero) },
            (Value::Float(x), Value::Float(y)) => Ok(Value::Float(x / y)),
            _ => Err(ValueError::FunctionNotAvailable("div", self.get_type(), other.get_type())),
        }
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Int(n) => write!(f, "{}", n),
            Value::Float(n) => write!(f, "{}", n),
            Value::Bool(b) => write!(f, "{}", b),
            _ => todo!(),
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

