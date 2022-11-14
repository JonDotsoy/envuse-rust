use crate::parser::span::Span;


#[derive(Debug)]
pub struct SyntaxError {
    pub message: String,
    pub span: Span,
}

impl SyntaxError {
    pub fn new<T: ToString>(message: T, span: Span) -> Self {
        Self {
            message: message.to_string(),
            span,
        }
    }
}
