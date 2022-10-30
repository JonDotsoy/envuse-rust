use self::{
    ast::{Expression, AST},
    tokenizer::Tokenizer,
};

pub mod ast;
pub mod tokenizer;

/// Parse source
pub fn parse<A: ToString>(payload: A) -> Result<Expression, ()> {
    let tokens = Tokenizer::parse(payload.to_string());
    Ok(AST::parse(tokens))
}
