use std::collections::VecDeque;

use thiserror;

use super::expression::*;

#[derive(Debug, thiserror::Error)]
enum ParseError {
    #[error("Unknown operator found {0}")]
    UnknownOperator(char),
    #[error("Invalid token found")]
    InvalidToken,
    #[error("Unclosed parenthesis")]
    UnclosedParenthesis,
    #[error("Invalid identifier")]
    InvalidIdentifier,
    #[error("Missing semicolon")]
    MissingSemicolon,
    #[error("Expected equals symbol")]
    EqualsExpected
}

#[derive(Debug)]
pub enum ParsedToken {
    DefineVar(String, Option<Expression>),
    SetVar(String, Expression),
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
    for (i, c) in input.char_indices() {
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

    fn validate_identifier(id: &str) -> Result<(), ParseError> {
        let first_c = id.chars().nth(0).unwrap();
        if !first_c.is_ascii_digit() &&
            (first_c.is_alphabetic() || first_c == '_') &&
            id[1..].chars().all(char::is_alphanumeric) {
            Ok(())
        } else {
            Err(ParseError::InvalidIdentifier)
        }
    }

    fn parse_expression(&mut self, min_bp: f32) -> Result<Expression, ParseError> {
        let mut lhs = match self.next() {
            Token::Number(s) => Expression::Number(s),
            Token::Op(_) => Err(ParseError::InvalidToken)?,
            Token::LeftParenth => { let expr = self.parse_expression(0.0)?;
                match self.next() {
                    Token::RightParenth => expr,
                    _ => return Err(ParseError::UnclosedParenthesis)
                }
            },
            Token::Identifier(s) => Expression::Identifier(s),
            _ => Err(ParseError::InvalidToken)?,
        };

        loop {
            let op = match self.peek() {
                Token::Number(_) => Err(ParseError::InvalidToken)?,
                Token::Op(op) => op,
                Token::RightParenth => break,
                Token::Semicolon => break,
                Token::Eof => Err(ParseError::MissingSemicolon)?,
                _ => Err(ParseError::MissingSemicolon)?,
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

    fn parse(&mut self) -> Result<Vec<ParsedToken>, ParseError> {
        let mut tokens = Vec::new();

        loop {
            let token = self.next();
            if token == Token::Eof {
                break;
            }

            match token {
                Token::Let => {
                    let next_token = self.next();
                    if let Token::Identifier(s) = next_token {
                        let s = unsafe { &*s };
                        Lexer::validate_identifier(s)?;

                        if self.next() != Token::Equals {
                            Err(ParseError::EqualsExpected)?
                        }

                        let expression = self.parse_expression(f32::NEG_INFINITY)?;
                        debug_assert_eq!(self.next(), Token::Semicolon);
                        tokens.push(ParsedToken::DefineVar(s.to_owned(), Some(expression)));
                    } else { unreachable!(); }
                },
                Token::Identifier(s) => {
                    let s = unsafe { &*s };
                    Lexer::validate_identifier(s)?;
                    
                    if self.next() != Token::Equals {
                        Err(ParseError::EqualsExpected)?
                    }

                    let expression = self.parse_expression(f32::NEG_INFINITY)?;
                    debug_assert_eq!(self.next(), Token::Semicolon);
                    tokens.push(ParsedToken::SetVar(s.to_owned(), expression));
                },
                _ => { println!("{}", token); todo!() },
            }
        };

        Ok(tokens)
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
            '=' => return Some(Token::Equals),
            '(' => return Some(Token::LeftParenth),
            ')' => return Some(Token::RightParenth),
            ';' => return Some(Token::Semicolon),
            _ => {},
        };
        if c.is_alphabetic() || c == '_' {
            *current_token_kind = TokenKind::Identifier;
            *start_i = char_i;
        }

        None
    }

    fn tokenize(&mut self, input: String) {
        self.input = input;
        self.tokens = VecDeque::new();
        
        // TODO: Use lazy splitting plz
        let mut words = split_words(&self.input).into_iter();

        let mut current_token_kind = TokenKind::None;
        let mut start_i = 0;

        while let Some((word, word_start_i)) = words.next() {
            let word_matched = match word {
                "let" => {
                    self.tokens.push_back(Token::Let);
                    let (identifier, _) = words.next().unwrap();
                    self.tokens.push_back(Token::Identifier(identifier as *const str));
                    true
                },
                _ => false,
            };

            if word_matched {
                continue;
            }

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
                        TokenKind::Identifier => {
                            match chars.peek() {
                                Some(peek_c) if peek_c.is_alphanumeric() || *peek_c == '_' => None,
                                _ => {
                                    current_token_kind = TokenKind::None;
                                    Some(Token::Identifier(&self.input[start_i..char_i+1] as *const str))
                                }
                            }
                        }
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
    use indoc::indoc;

    use super::*;

    use crate::parser::context::CTContext;
    use crate::types::{Value, ValueError};

    fn expression_helper(input: String, expected: &Value, expected_string: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut lexer = Lexer::new();

        lexer.tokenize(input);

        let expression = lexer.parse_expression(f32::NEG_INFINITY)?;

        assert_eq!(format!("{}", expression), expected_string);

        let value = expression.evaluate(&CTContext::new(), false)?;

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
        let input = "2 + 4 * 3;".to_owned();
        expression_helper(input, &Value::Int(14), "(+, 2, (*, 4, 3))")
    }

    #[test]
    fn test_multidigit() -> Result<(), Box<dyn std::error::Error>> {
        let input = "4500 / 50;".to_owned();
        expression_helper(input, &Value::Int(90), "(/, 4500, 50)")
    }

    #[test]
    fn test_implicit_conversion() -> Result<(), Box<dyn std::error::Error>> {
        let input = "2.5 + 3;".to_owned();
        expression_helper(input, &Value::Float(5.5), "(+, 2.5, 3)")?;

        let input = "2 + 3.5;".to_owned();
        expression_helper(input, &Value::Float(5.5), "(+, 2, 3.5)")
    }

    #[test]
    fn test_complex_expression() -> Result<(), Box<dyn std::error::Error>> {
        let input = "(8 + 42 / 7 * 3) / 2 - 5;".to_owned();
        expression_helper(input, &Value::Int(8), "(-, (/, (+, 8, (*, (/, 42, 7), 3)), 2), 5)")
    }

    fn test_var(ctx: &CTContext, name: &str, expected_val: Value) -> Result<(), String> {
        match ctx.variables.get(name) {
            Some(original_var) => {
                let mut var = original_var.clone();
                var.evaluate(ctx, false).unwrap();
                match var {
                    CTVariable::Value(val) => if val.equals(&expected_val).unwrap() {
                        Ok(())
                    } else {
                        Err(format!("Var {} doesn't have expected value: {} != {}", name, val, expected_val))
                    },
                    _ => Err("Value not reduced".to_owned()),
                }
            },
            None => Err("Expected var doesn't exist".to_owned()),
        }
    }

    #[test]
    fn test_variable() -> Result<(), Box<dyn std::error::Error>> {
        let mut lexer = Lexer::new();
        
        let input = indoc! {"
            let x = 5 * 2;
            x = 4;
            let y = 3 * 2;
        "}.to_owned();

        lexer.tokenize(input);

        let tokens = lexer.parse()?;

        println!("{:?}", tokens);

        panic!("ss");

        Ok(())
    }

    #[test]
    #[should_panic = "EqualsExpected"]
    fn test_equals_expected() {
        let mut lexer = Lexer::new();
        
        let input = indoc! {"
            let x a;
        "}.to_owned();

        lexer.tokenize(input);

        lexer.parse().unwrap();

        let ctx = &lexer.context;
        test_var(ctx, "x", Value::Int(6)).unwrap();
    }

    #[test]
    #[should_panic = "InvalidIdentifier"]
    fn test_invalid_identifier_leading_digit() {
        let mut lexer = Lexer::new();
        
        let input = indoc! {"
            let 2x = 2;
        "}.to_owned();

        lexer.tokenize(input);

        lexer.parse().unwrap();
    }

    #[test]
    #[should_panic = "InvalidIdentifier"]
    fn test_invalid_identifier_non_alphabetic() {
        let mut lexer = Lexer::new();
        
        let input = indoc! {"
            let ab} = 4;
        "}.to_owned();

        lexer.tokenize(input);

        lexer.parse().unwrap();
    }

    #[test]
    #[should_panic = "VariableWithIdentifierNotFound"]
    fn test_variable_not_found() {
        let mut lexer = Lexer::new();
        
        let input = indoc! {"
            let x = a;
        "}.to_owned();

        lexer.tokenize(input);

        lexer.parse().unwrap();

        let ctx = &lexer.context;
        test_var(ctx, "x", Value::Int(6)).unwrap();
    }

    #[test]
    #[should_panic = "MissingSemicolon"]
    fn test_missing_semicolon() {
        let mut lexer = Lexer::new();
        
        let input = indoc! {"
            let x = 4;
            let y = 3
            let z = 2;
        "}.to_owned();

        lexer.tokenize(input);

        lexer.parse().unwrap();
    }
}
