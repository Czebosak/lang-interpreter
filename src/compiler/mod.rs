use crate::compiler::context::CTContext;
use crate::config::{Config, OptimizationLevel};
use crate::parser::lexer::ParsedToken;
use crate::parser::expression::*;

pub mod instruction;
pub mod context;

pub struct Compiler {
    tokens: Vec<ParsedToken>,
}

impl Compiler {
    pub fn new(tokens: Vec<ParsedToken>) -> Self {
        Compiler { tokens }
    }

    pub fn compile(&mut self, config: &Config) {
        let ctx = CTContext::new();

        while let Some(token) = self.tokens.pop() {
            match token {
                ParsedToken::DefineVar(name, None) => ctx.register_variable(name, None),
                ParsedToken::DefineVar(name, Some(expr)) | ParsedToken::SetVar(name, expr) => {
                    let mut parsed = expr.parse(&ctx).unwrap();
                    if config.optimization >= OptimizationLevel::Standard {
                        parsed.const_fold(&ctx);
                    }

                    if let ParsedToken::DefineVar(_, _) = token {
                        ctx.register_variable(name, Some(parsed));
                    }
                },
            }
        }
    }
}
