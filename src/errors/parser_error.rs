use super::super::parser::span::Span;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct ParseError {
    pub message: String,
    pub span: Span,
}

impl ParseError {
    pub fn new<T>(message: T, span: Span) -> Self
    where
        T: ToString,
    {
        Self {
            message: message.to_string(),
            span,
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message.as_str())
    }
}

impl Error for ParseError {
    fn description(&self) -> &str {
        self.message.as_str()
    }
}
