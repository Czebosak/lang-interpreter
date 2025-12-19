use crate::types::{TypeValue, Value, ValueError};

#[derive(Debug, Clone)]
pub(super) enum Token {
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

pub(super) enum TokenKind {
    Number,
    Atoms,
    Op,
    Eof,
    None,
}

#[derive(Debug, Clone, Copy)]
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

pub(super) enum Expression {
    Number(*const str),
    Operation(Operation),
}

impl Expression {
    pub(super) fn evaluate(&self) -> Result<Value, ValueError> {
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

pub(super) struct Operation {
    pub op: Operator,
    pub expressions: Vec<Expression>,
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
