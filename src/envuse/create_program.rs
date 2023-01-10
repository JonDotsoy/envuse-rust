use super::super::parser::ast::AST;
use super::super::parser::tokenizer::Tokenizer;
use super::display_program_error::display_program_error;
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

    display_program_error(result_program, source, location_val)
}
