use super::super::errors::parser_error::ParseError;

use super::super::parser::ast::Variable;
use super::transformer_list::TransformerList;
use super::value_types::ValueType;
use std::collections::BTreeMap;

pub struct Parser;

impl Parser {
    pub fn to_parse_variable(
        transformer_list: &TransformerList,
        variable: &Variable,
        envs: &BTreeMap<String, Option<String>>,
    ) -> Result<ValueType, ParseError> {
        let default_type = String::from("String");
        let transform_type = variable
            .variable_type
            .clone()
            .unwrap_or(default_type)
            .to_lowercase();

        let transformer = match transformer_list.get(&transform_type) {
            Some(transformer) => transformer,
            _ => {
                do yeet ParseError::new(
                    format!("Type {} is not valid type", transform_type),
                    variable.span,
                )
            }
        };

        let value_env = envs.get(&variable.name).unwrap_or(&None);

        if variable.nullable && value_env.is_none() && variable.default_value.is_none() {
            return Ok(ValueType::Null);
        }

        let value_to_transform = if let Some(value_env) = value_env {
            value_env.to_string()
        } else if let Some(expression) = variable.default_value.as_ref() {
            if let Some(default_value) = expression.as_default_value() {
                default_value.value.to_string()
            } else {
                do yeet ParseError::new(format!("Expression cannot found error"), variable.span)
            }
        } else {
            do yeet ParseError::new(
                format!("{} value cannot be null", &variable.name),
                variable.span,
            )
        };

        Ok(transformer.as_ref().parse(value_to_transform))
    }
}
