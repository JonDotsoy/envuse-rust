use super::{transformer_list::TransformerList, value_types::ValueType};
use crate::parser::ast::Variable;
use std::collections::BTreeMap;

pub struct Parser;

impl Parser {
    pub fn to_parse_variable(
        transformer_list: &TransformerList,
        variable: &Variable,
        envs: &BTreeMap<String, Option<String>>,
    ) -> ValueType {
        let default_type = String::from("String");
        let transform_type = variable.variable_type.clone().unwrap_or(default_type).to_lowercase();
        let transformer = transformer_list
            .get(&transform_type)
            .expect(format!("Type {} is not valid type", transform_type).as_str());

        let value_env = envs.get(&variable.name).unwrap_or(&None);

        if variable.nullable && value_env.is_none() && variable.default_value.is_none() {
            return ValueType::Null;
        }

        let value_to_tansform = if let Some(value_env) = value_env {
            value_env.to_string()
        } else if let Some(expression) = variable.default_value.as_ref() {
            if let Some(default_value) = expression.as_default_value() {
                default_value.value.to_string()
            } else {
                panic!("Expression cannot found error");
            }
        } else {
            panic!("{}", format!("Cannot found value for {}", &variable.name));
        };

        transformer.as_ref().parse(value_to_tansform)
    }
}
