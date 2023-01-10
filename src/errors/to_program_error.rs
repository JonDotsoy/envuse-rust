use super::parser_error::ParseError;

pub trait ToProgramError {}

impl ToProgramError for ParseError {}
