use crate::{
    errors::{parser_error::ParseError, program_error::ProgramError},
    syntax_error::SyntaxError,
};

use std::error::Error;

pub fn display_program_error<A, T: ToString>(
    result: Result<A, Box<dyn Error>>,
    source: T,
    location_val: Option<String>,
) -> Result<A, Box<dyn Error>> {
    match result {
        Err(error) if error.is::<SyntaxError>() => {
            let syntax_error = error.downcast_ref::<SyntaxError>().unwrap();
            do yeet ProgramError {
                message: format!("SyntaxError: {}", syntax_error.message),
                span: Some(syntax_error.span.clone()),
                source: source.to_string(),
                location: location_val,
                cause: Some(error),
            }
        }
        Err(error) if error.is::<ParseError>() => {
            let syntax_error = error.downcast_ref::<ParseError>().unwrap();
            do yeet ProgramError {
                message: format!("ParseError: {}", syntax_error.message),
                span: Some(syntax_error.span.clone()),
                source: source.to_string(),
                location: location_val,
                cause: Some(error),
            }
        }
        result_program => result_program,
    }
}
