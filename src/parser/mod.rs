use crate::syntax_error::SyntaxError;

use self::{
    ast::{Expression, AST},
    tokenizer::Tokenizer,
};

pub mod ast;
pub mod span;
pub mod tokenizer;

/// Parse source
pub fn parse<A: ToString>(payload: A) -> Result<Expression, SyntaxError> {
    let tokens = Tokenizer::parse(payload.to_string())?;
    Ok(AST::parse(tokens)?)
}
