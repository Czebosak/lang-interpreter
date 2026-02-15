use crate::compiler::context::{CTContext, CTVariable};
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
pub enum Expression {
    Number(*const str),
    Identifier(*const str),
    Operation(Operation),
}

impl Expression {
    pub fn parse(&self, ctx: &CTContext) -> Result<ParsedExpression, ValueError> {
        match self {
            Expression::Number(s) => {
                let n;
                unsafe { n = (**s).parse::<f64>().unwrap(); }

                if n.fract() == 0.0 {
                    Ok(ParsedExpression::Value(Value::Int(n as i64)))
                } else {
                    Ok(ParsedExpression::Value(Value::Float(n)))
                }
            },
            Expression::Identifier(s) => /* match ctx.lookup_identifier(unsafe { &**s }) { } */ {
                todo!()
            },
            Expression::Operation(op) => {
                todo!()
            }
        }
    }

    /* pub fn evaluate(&self, ctx: &CTContext, const_fold: bool) -> Result<Value, ValueError> {
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
    } */

    /* pub fn const_fold(ctx: &CTContextStack) -> Result<ParsedExpression, ValueError> {
        match self {
            Expression::Number(s) => {
                let n;
                unsafe { n = (**s).parse::<f64>().unwrap(); }

                if n.fract() == 0.0 {
                    Ok(ParsedExpression::Value(Value::Int(n as i64)))
                } else {
                    Ok(ParsedExpression::Value(Value::Float(n)))
                }
            },
            Expression::Identifier(s) => match ctx.variables.get(unsafe { &**s }) {
                Some(var) => {
                    match var {
                        CTVariable::Expression(expr) => expr.(ctx),
                        CTVariable::Value(val) => Ok(val.clone()),
                    }
                },
                None => Err(ValueError::VariableWithIdentifierNotFound),
            },
            Expression::Operation(op) => op.evaluate(ctx, const_fold),
        }
    } */
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
pub enum ParsedExpression<'a> {
    Value(Value),
    Variable(*mut CTVariable<'a>),
    ParsedOperation(ParsedOperation<'a>),
}

impl ParsedExpression<'_> {
    pub fn const_fold(&mut self, ctx: &CTContext) -> Option<Value> {
        match self {
            ParsedExpression::Value(val) => Some(*val),
            ParsedExpression::Variable(var) => unsafe { **var }.const_fold(ctx),
            ParsedExpression::ParsedOperation(op) => op.const_fold(ctx),
        }
    }
}

impl std::fmt::Display for ParsedExpression<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParsedExpression::Value(val) => write!(f, "{}", val),
            _ => todo!()
            //ParsedExpression::ParsedOperation(val) => write!(f, "{}", val),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Operation {
    pub op: Operator,
    pub expressions: Vec<Expression>,
}

impl Operation {
    fn parse(&mut self, ctx: &CTContext) -> Result<ParsedOperation<'_>, ValueError> {
        let mut expressions = Vec::with_capacity(2);
        
        for expr in &self.expressions {
            expressions.push(expr.parse(ctx)?);
        }

        Ok(ParsedOperation {
            op: self.op,
            expressions,
        })
    }
}

#[derive(Debug, Clone)]
pub struct ParsedOperation<'a> {
    op: Operator,
    expressions: Vec<ParsedExpression<'a>>,
}

impl ParsedOperation<'_> {
    fn evaluate(&self, val1: Value, val2: Value) -> Result<Value, ValueError> {
        match self.op {
            Operator::Plus  => val1.add(&val2),
            Operator::Minus => val1.sub(&val2),
            Operator::Star  => val1.mul(&val2),
            Operator::Slash => val1.div(&val2),
            _ => todo!(),
        }
    }

    pub fn const_fold(&mut self, ctx: &CTContext) -> Option<Result<Value, ValueError>> {
        let mut values = [None, None];

        for i in 0..2 {
            values[i] = self.expressions[i].const_fold(ctx);
        }

        if let [Some(val1), Some(val2)] = values {
            return Some(self.evaluate(val1, val2));
        }

        None
    }
}
