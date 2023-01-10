use crate::errors::program_error::ProgramError;

use crate::syntax_error::SyntaxError;

use super::super::parser::ast::AST;
use super::super::parser::tokenizer::Tokenizer;
use super::program::Program;

pub fn create_program<T: ToString>(
    source: T,
    location: Option<T>,
) -> Result<Program, Box<dyn std::error::Error>> {
    let location_val = location.map(|t| t.to_string());

    let result_program: Result<Program, Box<dyn std::error::Error>> = try {
        Program {
            location: location_val.clone(),
            source: source.to_string(),
            ast: AST::parse(Tokenizer::parse(source.to_string())?)?,
        }
    };

    match result_program {
        Err(error) if error.is::<SyntaxError>() => {
            let syntax_error = error.downcast::<SyntaxError>().unwrap();
            do yeet ProgramError {
                message: format!("SyntaxError: {}", syntax_error.message),
                span: Some(syntax_error.span.clone()),
                source: source.to_string(),
                location: location_val,
                cause: Some(*syntax_error),
            }
        }
        result_program => result_program,
    }
}
