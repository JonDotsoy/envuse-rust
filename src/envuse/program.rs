use super::super::envuse::to_envs::ToEnvs;
use super::super::parser::ast::Expression;
use super::super::transformers::kinds::boolean_transform::BooleanTransform;
use super::super::transformers::kinds::number_transform::NumberTransform;
use super::super::transformers::kinds::string_transform::StringTransform;
use super::super::transformers::parser::Parser;
use super::super::transformers::transformer_list::TransformerList;
use super::super::transformers::value_types::ValueType;
use super::to_custom_transformers::ToCustomTransformers;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// The program is a structure content the filename and the source
#[derive(Debug, Serialize, Deserialize)]
pub struct Program {
    pub location: Option<String>,
    pub source: String,
    pub ast: Expression,
}

impl Program {
    pub fn parse<T, D>(
        &self,
        values: T,
        custom_transformers: D,
    ) -> Result<BTreeMap<String, ValueType>, Box<dyn std::error::Error>>
    where
        T: ToEnvs,
        D: ToCustomTransformers,
    {
        let mut transformer_list: TransformerList = Default::default();

        transformer_list.insert("unknown", Box::new(StringTransform));
        transformer_list.insert("str", Box::new(StringTransform));
        transformer_list.insert("string", Box::new(StringTransform));
        transformer_list.insert("number", Box::new(NumberTransform));
        transformer_list.insert("bool", Box::new(BooleanTransform));
        transformer_list.insert("boolean", Box::new(BooleanTransform));

        for key in custom_transformers.to_vec() {
            transformer_list.insert(key, Box::new(StringTransform));
        }

        let envs_values = values.to_envs();

        let document = &self
            .ast
            .as_document()
            .expect("AST Parser error, the ast expressions is not Expression::Document");

        let expressions = &document.elements;

        let mut configs: BTreeMap<String, ValueType> = BTreeMap::new();

        for expression in expressions {
            let variable = expression
                .as_variable()
                .expect("Expression is not supported");

            let value = Parser::to_parse_variable(&transformer_list, &variable, &envs_values)?;

            configs.insert(variable.name.to_string(), value);
        }

        Ok(configs)
    }
}
