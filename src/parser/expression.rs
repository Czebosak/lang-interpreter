use crate::parser::context::{CTContext, CTVariable};
use crate::types::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum Token {
    Number(*const str),
    Op(Operator),
    LeftParenth,
    RightParenth,
    Eof,
    Semicolon,
    Equals,
    Let,
    Identifier(*const str),
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Number(ptr) => unsafe { write!(f, "Token::Number({})", &**ptr) },
            Token::Identifier(ptr) => unsafe { write!(f, "Token::Identifier({})", &**ptr) },
            _ => write!(f, "{:?}", self),
        }
    }
}

pub(super) enum TokenKind {
    Number,
    Op,
    Eof,
    Identifier,
    None,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Operator {
    Plus,
    Minus,
    Star,
    Slash,
    Pipe,
    Ampersand,
    Bang,
}

impl std::fmt::Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Operator::Plus  => '+',
            Operator::Minus => '-',
            Operator::Star  => '*',
            Operator::Slash => '/',
            Operator::Pipe  => '|',
            Operator::Ampersand => '&',
            Operator::Bang  => '!',
        };

        write!(f, "{}", c)
    }
}

#[derive(Debug, Clone)]
pub(super) enum Expression {
    Number(*const str),
    Identifier(*const str),
    Operation(Operation),
}

impl Expression {
    pub(super) fn evaluate(&self, ctx: &CTContext, const_fold: bool) -> Result<Value, ValueError> {
        match self {
            Expression::Number(s) => {
                let n;
                unsafe { n = (**s).parse::<f64>().unwrap(); }

                if n.fract() == 0.0 {
                    Ok(Value::Int(n as i64))
                } else {
                    Ok(Value::Float(n))
                }
            },
            Expression::Identifier(s) => match ctx.variables.get(unsafe { &**s }) {
                Some(var) => {
                    match var {
                        CTVariable::Expression(expr) => expr.evaluate(ctx, const_fold),
                        CTVariable::Value(val) => Ok(val.clone()),
                    }
                },
                None => Err(ValueError::VariableWithIdentifierNotFound),
            },
            Expression::Operation(op) => op.evaluate(ctx, const_fold),
        }
    }
}

impl std::fmt::Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::Number(s) | Expression::Identifier(s)  => unsafe { write!(f, "{}", &**s) },
            Expression::Operation(op) => {
                write!(f, "({}, {}, {})", op.op, op.expressions[0], op.expressions[1])
            }
        }
    }
}

#[derive(Debug, Clone)]
pub(super) struct Operation {
    pub op: Operator,
    pub expressions: Vec<Expression>,
}

impl Operation {
    fn process<F>(&self, op: F, ctx: &CTContext, const_fold: bool) -> Result<Value, ValueError> where F: Fn(&Value, &Value) -> Result<Value, ValueError> {
        op(&self.expressions[0].evaluate(ctx, const_fold)?, &self.expressions[1].evaluate(ctx, const_fold)?)
    }

    fn evaluate(&self, ctx: &CTContext, const_fold: bool) -> Result<Value, ValueError> {
        match self.op {
            Operator::Plus  => self.process(Value::add, ctx, const_fold),
            Operator::Minus => self.process(Value::sub, ctx, const_fold),
            Operator::Star  => self.process(Value::mul, ctx, const_fold),
            Operator::Slash => self.process(Value::div, ctx, const_fold),
            _ => todo!(),
        }
    }
}
