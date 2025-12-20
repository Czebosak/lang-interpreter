use std::collections::HashMap;

use crate::types::*;
use crate::parser::expression::Expression;

#[derive(Clone)]
pub enum CTVariable {
    Value(Value),
    Expression(Expression),
}

impl CTVariable {
    pub fn is_constant(&self) -> bool {
        match self {
            CTVariable::Value(_) => true,
            CTVariable::Expression(_) => false,
        }
    }

    pub fn evaluate(&mut self, ctx: &CTContext, const_fold: bool) -> Result<(), ValueError> {
        match self {
            CTVariable::Expression(expr) => *self = CTVariable::Value(expr.evaluate(ctx, const_fold)?),
            CTVariable::Value(_) => {},
        };
        Ok(())
    }
}

impl std::fmt::Display for CTVariable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CTVariable::Value(val) => write!(f, "{}", val),
            CTVariable::Expression(expr) => write!(f, "{}", expr),
        }
    }
}

pub struct CTContext {
    pub variables: HashMap<String, CTVariable>,
}

impl CTContext {
    pub fn new() -> CTContext {
        CTContext {
            variables: HashMap::new(),
        }
    }
}
