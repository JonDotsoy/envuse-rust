use super::super::envuse::program::Program;
use super::parser_error::ParseError;
use crate::parser::span::Span;
use crate::syntax_error::SyntaxError;
use crate::utils::display_syntax::{DisplaySyntax, DisplaySyntaxDebugOptions};
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct ProgramError {
    pub message: String,
    pub span: Option<Span>,
    pub source: String,
    pub location: Option<String>,
    pub cause: Option<Box<dyn Error>>,
}

impl ProgramError {
    pub fn get_message(&self) -> String {
        let mut debug_options = DisplaySyntaxDebugOptions::new();
        debug_options.location = self.location.clone();

        match &self.cause {
            Some(error) if error.is::<SyntaxError>() => {
                let syntax_error = error.downcast_ref::<SyntaxError>().unwrap();
                let display_syntax = DisplaySyntax::new(
                    format!("SyntaxError: {}", syntax_error.message),
                    syntax_error.span.clone(),
                );

                display_syntax.debug_payload_configurable(&self.source, &debug_options)
            }
            Some(error) if error.is::<ParseError>() => {
                let parse_error = error.downcast_ref::<ParseError>().unwrap();
                let display_syntax = DisplaySyntax::new(
                    format!("ParseError: {}", parse_error.message),
                    parse_error.span.clone(),
                );

                display_syntax.debug_payload_configurable(&self.source, &debug_options)
            }
            _ => self.message.to_string(),
        }
    }
}

impl fmt::Display for ProgramError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.get_message().to_string())
    }
}

impl std::error::Error for ProgramError {}

impl From<(&Program, &str)> for ProgramError {
    fn from(arg: (&Program, &str)) -> Self {
        let (program, str) = arg;
        ProgramError {
            message: String::from(str),
            span: None,
            source: program.source.clone(),
            location: program.location.clone(),
            cause: None,
        }
    }
}
