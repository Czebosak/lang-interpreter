use std::collections::HashMap;

use crate::types::*;
use crate::parser::expression::ParsedExpression;

#[derive(Clone)]
pub struct CTVariable<'a> {
    expression: Option<ParsedExpression<'a>>,
}

impl CTVariable<'_> {
    pub fn is_const(&self) -> bool {
        self.expression.is_const()
    }
}

impl std::fmt::Display for CTVariable<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        //write!(f, "{}", self.expression)
        todo!()
    }
}

pub struct CTContextFrame<'a> {
    pub variables: HashMap<String, Option<CTVariable<'a>>>,
}

impl<'a> CTContextFrame<'a> {
    pub fn new() -> CTContextFrame<'a> {
        CTContextFrame {
            variables: HashMap::new(),
        }
    }
}

enum Definition<'a> {
    VarDefinition(&'a CTVariable<'a>),
}

pub struct CTContext<'a> {
    data: Vec<CTContextFrame<'a>>,
}

impl<'a> CTContext<'a> {
    pub fn new() -> CTContext<'a> {
        CTContext { data: Vec::new() }
    }

    pub fn push(&mut self, frame: CTContextFrame<'a>) {
        self.data.push(frame);
    }

    pub fn pop(&mut self) {
        let _ = self.data.pop();
    }

    pub fn lookup_identifier(&self, id: &str) -> Option<Definition> {
        for frame in &self.data {
            if let Some(var_opt) = frame.variables.get(id) {
                return Some(Definition::VarDefinition(var));
            }
        }
        None
    }

    pub fn register_variable(&mut self, id: String, expr: Option<ParsedExpression>) {
        self.data.last().unwrap().variables.insert(id, expr);
    }
}
