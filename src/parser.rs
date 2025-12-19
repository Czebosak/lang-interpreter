use std::collections::VecDeque;

use thiserror;

use crate::types::{TypeValue, Value, ValueError};

#[derive(Debug, thiserror::Error)]
enum ParseError {
    #[error("Unknown operator found {0}")]
    UnknownOperator(char),
    #[error("Invalid token found")]
    InvalidToken,
    #[error("Unclosed Parenthesis")]
    UnclosedParenthesis
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

enum Expression {
    Number(*const str),
    Operation(Operation),
}

impl Expression {
    fn evaluate(&self) -> Result<Value, ValueError> {
        match self {
            Expression::Number(s) => {
                let n;
                unsafe { n = (**s).parse::<f64>().unwrap(); }

                println!("{}", n);

                if n.fract() == 0.0 {
                    Ok(Value::Int(n as i64))
                } else {
                    Ok(Value::Float(n))
                }
            },
            Expression::Operation(op) => op.evaluate(),
        }
    }
}

impl std::fmt::Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::Number(s) => unsafe { write!(f, "{}", &**s) },
            Expression::Operation(op) => {
                write!(f, "({}, {}, {})", op.op, op.expressions[0], op.expressions[1])
            }
        }
    }
}

struct Operation {
    op: Operator,
    expressions: Vec<Expression>,
}

impl Operation {
    fn process<F>(&self, op: F) -> Result<Value, ValueError> where F: Fn(&Value, &Value) -> Result<Value, ValueError> {
        op(&self.expressions[0].evaluate()?, &self.expressions[1].evaluate()?)
    }

    fn evaluate(&self) -> Result<Value, ValueError> {
        match self.op {
            Operator::Plus  => self.process(Value::add),
            Operator::Minus => self.process(Value::sub),
            Operator::Star  => self.process(Value::mul),
            Operator::Slash => self.process(Value::div),
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
    pub fn new() -> Lexer {
        Lexer {
            input: String::new(),
            tokens: VecDeque::new(),
        }
    }

    fn next(&mut self) -> Token {
        self.tokens.pop_front().unwrap_or(Token::Eof)
    }

    fn peek(&self) -> Token {
        self.tokens.front().unwrap_or(&Token::Eof).clone()
    }

    fn parse_expression(&mut self, min_bp: f32) -> Result<Expression, ParseError> {
        let mut lhs = match self.next() {
            Token::Number(s) => Expression::Number(s),
            Token::Op(_) => Err(ParseError::InvalidToken)?,
            Token::LeftParenth => { let expr = self.parse_expression(0.0)?;
                match self.next() {
                    Token::RightParenth => expr, _ => return Err(ParseError::UnclosedParenthesis)
                }
            },
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

    fn expression_helper(input: String, expected: &Value, expected_string: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut lexer = Lexer::new();

        lexer.tokenize(input);

        let expression = lexer.parse_expression(f32::NEG_INFINITY)?;

        assert_eq!(format!("{}", expression), expected_string);

        let value = expression.evaluate()?;

        match value.equals(expected) {
            Ok(true) => Ok(()),
            Ok(false) => panic!("Values mismatched: {} != {}", value, expected),
            Err(e) => match e {
                ValueError::FunctionNotAvailable(_, t1, t2) if t1 != t2 => {
                    panic!("Mismatched types: {} (expected {})", t1, t2)
                },
                _ => Err(Box::new(e)),
            },
        }
    }

    #[test]
    fn test_expression() -> Result<(), Box<dyn std::error::Error>> {
        let input = "2 + 4 * 3".to_owned();
        expression_helper(input, &Value::Int(14), "(+, 2, (*, 4, 3))")
    }

    #[test]
    fn test_multidigit() -> Result<(), Box<dyn std::error::Error>> {
        let input = "4500 / 50".to_owned();
        expression_helper(input, &Value::Int(90), "(/, 4500, 50)")
    }

    #[test]
    fn test_implicit_conversion() -> Result<(), Box<dyn std::error::Error>> {
        let input = "2.5 + 3".to_owned();
        expression_helper(input, &Value::Float(5.5), "(+, 2.5, 3)")?;

        let input = "2 + 3.5".to_owned();
        expression_helper(input, &Value::Float(5.5), "(+, 2, 3.5)")
    }

    #[test]
    fn test_complex_expression() -> Result<(), Box<dyn std::error::Error>> {
        let input = "(8 + 42 / 7 * 3) / 2 - 5".to_owned();
        expression_helper(input, &Value::Int(8), "(-, (/, (+, 8, (*, (/, 42, 7), 3)), 2), 5)")
    }
}
