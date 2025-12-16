use thiserror;

use crate::types::{TypeValue, Value, ValueError};

#[derive(Debug, thiserror::Error)]
enum ParseError {
    #[error("Unknown operator found {0}")]
    UnknownOperator(char),
    #[error("Invalid token found")]
    InvalidToken,
}

#[derive(Clone)]
enum Token {
    Atom(char),
    Op(char),
    Eof,
}

enum Expression {
    Atom(char),
    Operation(Operation),
}

impl Expression {
    fn evaluate(&self) -> Result<Value, ValueError> {
        match self {
            Expression::Atom(a) => {
                let n = match a.to_digit(10) {
                    Some(n) => n as i64,
                    None => Err(ValueError::FunctionNotAvailable("a", TypeValue::Bool, TypeValue::Float))?,
                };
                Ok(Value::Int(n))
            },
            Expression::Operation(op) => op.evaluate(),
        }
    }
}

struct Operation {
    op: char,
    expressions: Vec<Expression>,
}

impl Operation {
    fn evaluate(&self) -> Result<Value, ValueError> {
        match self.op {
            '+' => self.expressions[0].evaluate()?.add(&self.expressions[1].evaluate()?),
            '*' => self.expressions[0].evaluate()?.mul(&self.expressions[1].evaluate()?),
            _ => unreachable!(),
        }
    }
}

fn get_binding_power(op: char) -> Option<(f32, f32)> {
    match op {
        '+' | '-' => Some((1.0, 1.1)),
        '*' | '/' => Some((1.0, 1.1)),
        _ => None,
    }
}

pub struct Lexer {
    tokens: Vec<Token>,
}

impl Lexer {
    fn next(&mut self) -> Token {
        self.tokens.pop().unwrap_or(Token::Eof)
    }

    fn peek(&self) -> Token {
        self.tokens.last().unwrap_or(&Token::Eof).clone()
    }

    fn parse_expression(&mut self, min_bp: f32) -> Result<Expression, ParseError> {
        let mut lhs = match self.next() {
            Token::Atom(a) => Expression::Atom(a),
            Token::Op(op) => if op == '(' { self.parse_expression(0.0)? } else { Err(ParseError::InvalidToken)? },
            _ => Err(ParseError::InvalidToken)?,
        };

        loop {
            let op = match self.peek() {
                Token::Atom(a) => Err(ParseError::InvalidToken)?,
                Token::Op(op) => if op == ')' { break; } else { op },
                Token::Eof => break,
            };

            let bp = match get_binding_power(op) {
                Some(bp) => bp,
                None => Err(ParseError::UnknownOperator(op))?,
            };

            if bp.0 < min_bp {
                break;
            }

            self.next();

            let rhs = self.parse_expression(bp.1)?;

            lhs = Expression::Operation(Operation {
                op,
                expressions: vec![lhs, rhs],
            });
        }

        Ok(lhs)
    }

    fn tokenize(&mut self, input: &str) {
        self.tokens = input
            .chars()
            .filter(|c| !c.is_whitespace())
            .map(|c| if c.is_alphanumeric() { Token::Atom(c) } else { Token::Op(c) })
            .collect();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expression() -> Result<(), Box<dyn std::error::Error>> {
        let mut lexer = Lexer { tokens: vec![] };

        let input = "2 + 4 * 3";
        lexer.tokenize(input);

        let expression = lexer.parse_expression(f32::NEG_INFINITY)?;

        let value = expression.evaluate()?;

        if let Value::Int(n) = value {
            assert_eq!(n, 14);
        } else {
            panic!("Not int");
        }

        Ok(())
    }
}
