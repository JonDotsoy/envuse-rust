use crate::parser::span::Span;

#[derive(Debug, Clone)]
pub struct SyntaxError {
    pub message: String,
    pub span: Span,
}

impl std::fmt::Display for SyntaxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for SyntaxError {}

pub trait ErrorWithSpan {
    fn get_message(&self) -> Span;
    fn get_span(&self) -> Span;
}

impl SyntaxError {
    pub fn new<T: ToString>(message: T, span: Span) -> Self {
        Self {
            message: message.to_string(),
            span,
        }
    }
}
