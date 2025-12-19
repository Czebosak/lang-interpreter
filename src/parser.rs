use std::collections::VecDeque;

use thiserror;

use crate::types::{TypeValue, Value, ValueError};

#[derive(Debug, thiserror::Error)]
enum ParseError {
    #[error("Unknown operator found {0}")]
    UnknownOperator(char),
    #[error("Invalid token found")]
    InvalidToken,
}

#[derive(Debug, Clone)]
enum Token {
    Number(*const str),
    Op(Operator),
    LeftParenth,
    RightParenth,
    Eof,
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Number(ptr) => unsafe { write!(f, "Token::Number({})", &**ptr) },
            _ => write!(f, "{:?}", self),
        }
    }
}

enum TokenKind {
    Number,
    Atoms,
    Op,
    Eof,
    None,
}

#[derive(Debug, Clone, Copy)]
enum Operator {
    Plus,
    Minus,
    Star,
    Slash,
    Pipe,
    Ampersand,
    Bang,
}

enum Expression {
    Number(*const str),
    Operation(Operation),
}

impl Expression {
    fn evaluate(&self) -> Result<Value, ValueError> {
        match self {
            Expression::Number(s) => {
                unsafe {
                    let n = (**s).parse::<f64>().unwrap();

                    if n.fract() == 0.0 {
                        Ok(Value::Int(n as i64))
                    } else {
                        Ok(Value::Float(n))
                    }
                }
            },
            Expression::Operation(op) => op.evaluate(),
        }
    }
}

struct Operation {
    op: Operator,
    expressions: Vec<Expression>,
}

impl Operation {
    fn evaluate(&self) -> Result<Value, ValueError> {
        match self.op {
            Operator::Plus => self.expressions[0].evaluate()?.add(&self.expressions[1].evaluate()?),
            Operator::Star => self.expressions[0].evaluate()?.mul(&self.expressions[1].evaluate()?),
            _ => todo!(),
        }
    }
}

fn get_binding_power(op: Operator) -> (f32, f32) {
    match op {
        Operator::Plus | Operator::Minus => (1.0, 1.1),
        Operator::Star | Operator::Slash => (2.0, 2.1),
        _ => todo!(),
    }
}

fn split_words(input: &str) -> Vec<(&str, usize)> {
    let mut words = Vec::new();
    
    let mut start_i = 0;
    for (i, c) in input.chars().enumerate() {
        if c.is_whitespace() {
            words.push((&input[start_i..i], start_i));
            start_i = i + 1;
        }
    }
    if start_i < input.len() {
        words.push((&input[start_i..], start_i));
    }

    words
}

pub struct Lexer {
    tokens: VecDeque<Token>,
    input: String,
}

impl Lexer {
    fn next(&mut self) -> Token {
        self.tokens.pop_front().unwrap_or(Token::Eof)
    }

    fn peek(&self) -> Token {
        self.tokens.front().unwrap_or(&Token::Eof).clone()
    }

    fn parse_expression(&mut self, min_bp: f32) -> Result<Expression, ParseError> {
        println!("{}", self.peek());
        let mut lhs = match self.next() {
            Token::Number(s) => Expression::Number(s),
            Token::Op(_) => Err(ParseError::InvalidToken)?,
            Token::LeftParenth => self.parse_expression(0.0)?,
            _ => Err(ParseError::InvalidToken)?,
        };

        loop {
            let op = match self.peek() {
                Token::Number(_) => Err(ParseError::InvalidToken)?,
                Token::Op(op) => op,
                Token::RightParenth => break,
                Token::Eof => break,
                _ => Err(ParseError::InvalidToken)?,
            };

            let bp = get_binding_power(op);

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

    fn handle_char_no_token(c: char, current_token_kind: &mut TokenKind, char_i: usize, start_i: &mut usize) -> Option<Token> {
        if c.is_numeric() || c == '.' {
            *current_token_kind = TokenKind::Number;
            *start_i = char_i;
            return None;
        };
        match c {
            '+' => return Some(Token::Op(Operator::Plus)),
            '-' => return Some(Token::Op(Operator::Minus)),
            '*' => return Some(Token::Op(Operator::Star)),
            '/' => return Some(Token::Op(Operator::Slash)),
            '|' => return Some(Token::Op(Operator::Pipe)),
            '&' => return Some(Token::Op(Operator::Ampersand)),
            '!' => return Some(Token::Op(Operator::Bang)),
            '(' => return Some(Token::LeftParenth),
            ')' => return Some(Token::RightParenth),
            _ => {},
        };

        None
    }

    fn tokenize(&mut self, input: String) {
        self.input = input;
        self.tokens = VecDeque::new();
        
        // TODO: Use lazy splitting plz
        let words = split_words(&self.input);

        let mut current_token_kind = TokenKind::None;
        let mut start_i = 0;

        for (word, word_start_i) in words {
            let mut chars = word.chars().peekable();
            let mut char_i = word_start_i;

            while let Some(c) = chars.next() {
                let mut token_opt = if let TokenKind::None = current_token_kind {
                    Lexer::handle_char_no_token(c, &mut current_token_kind, char_i, &mut start_i)
                } else {
                    None
                };

                if token_opt.is_none() {
                    token_opt = match current_token_kind {
                        TokenKind::Number => {
                            // Check if next char is valid number
                            // else finish creating the token
                            match chars.peek() {
                                Some(peek_c) if peek_c.is_numeric() || *peek_c == '.' || *peek_c == '_' => None,
                                _ => {
                                    current_token_kind = TokenKind::None;
                                    Some(Token::Number(&self.input[start_i..char_i+1] as *const str))
                                },
                            }
                        },
                        _ => unreachable!(),
                    };
                }
                if let Some(token) = token_opt {
                    self.tokens.push_back(token);
                }
                char_i += 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expression() -> Result<(), Box<dyn std::error::Error>> {
        let mut lexer = Lexer { tokens: VecDeque::new(), input: String::new() };

        let input = "2 + 4 * 3".to_owned();
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
