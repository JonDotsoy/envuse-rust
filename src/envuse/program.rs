use crate::errors::program_error::ProgramError;

use super::super::parser::ast::Expression;
use super::super::transformers::kinds::boolean_transform::BooleanTransform;
use super::super::transformers::kinds::number_transform::NumberTransform;
use super::super::transformers::kinds::string_transform::StringTransform;
use super::super::transformers::parser::Parser;
use super::super::transformers::transformer_list::TransformerList;
use super::super::transformers::value_types::ValueType;
use super::to_custom_transformers::ToCustomTransformers;
use super::{super::envuse::to_envs::ToEnvs, display_program_error::display_program_error};
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
        display_program_error(
            self.parse_unwrap(values, custom_transformers),
            self.source.clone(),
            self.location.clone(),
        )
    }

    fn parse_unwrap<T, D>(
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

        let document = match self.ast.as_document() {
            Some(document) => document,
            _ => {
                do yeet ProgramError::from((
                    self,
                    "AST Parser error, the ast expressions is not Expression::Document",
                ))
            }
        };

        let expressions = &document.elements;

        let mut configs: BTreeMap<String, ValueType> = BTreeMap::new();

        for expression in expressions {
            let variable = match expression.as_variable() {
                Some(variable) => variable,
                _ => do yeet ProgramError::from((self, "Expression is not supported")),
            };

            let value = Parser::to_parse_variable(&transformer_list, &variable, &envs_values)?;

            configs.insert(variable.name.to_string(), value);
        }

        Ok(configs)
    }
}
