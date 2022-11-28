use crate::{
    parser::{
        ast::{Expression, AST},
        span::Span,
        tokenizer::Tokenizer,
    },
    syntax_error::{DebugOptions, SyntaxError},
    transformers::{
        kinds::{
            boolean_transform::BooleanTransform, number_transform::NumberTransform,
            string_transform::StringTransform,
        },
        parser::Parser,
        transformer_list::TransformerList,
        value_types::ValueType,
    },
};
#[cfg(feature = "with-js")]
use js_sys::SyntaxError as JSSyntaxError;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
#[cfg(feature = "with-js")]
use wasm_bindgen::JsValue;

pub struct Evaluate;

pub struct EvaluateOptions;

pub struct ParseOptions {
    pub environment_values: BTreeMap<String, String>,
}

#[derive(Debug)]
pub struct ProgramError {
    pub message: String,
    pub span: Option<Span>,
    pub source: String,
    pub location: Option<String>,
    pub cause: Option<SyntaxError>,
}

impl ProgramError {
    fn create_prepare_context<T: ToString, F: FnOnce() -> Result<Program, SyntaxError>>(
        location: Option<T>,
        source: T,
        handler: F,
    ) -> Result<Program, ProgramError> {
        match handler() {
            Ok(program) => Ok(program),
            Err(error) => Err(ProgramError {
                message: format!("SyntaxError: {}", error.message),
                source: source.to_string(),
                span: Some(error.span.clone()),
                location: location.map(|t| t.to_string()),
                cause: Some(error),
            }),
        }
    }

    pub fn get_message(self) -> String {
        let mut debug_options = DebugOptions::new();
        debug_options.location = self.location.clone();

        self.cause
            .unwrap()
            .debug_payload_configurable(&self.source, &debug_options)
    }
}

#[cfg(feature = "with-js")]
impl From<ProgramError> for JSSyntaxError {
    fn from(error: ProgramError) -> Self {
        Self::new(&error.get_message().clone())
    }
}

#[cfg(feature = "with-js")]
impl From<ProgramError> for JsValue {
    fn from(error: ProgramError) -> Self {
        Self::from(JSSyntaxError::from(error))
    }
}

/// The program is a structure content the filename and the source
#[derive(Debug, Serialize, Deserialize)]
pub struct Program {
    pub location: Option<String>,
    pub source: String,
    pub ast: Expression,
}

pub trait ToEnvs {
    fn to_envs(self) -> BTreeMap<String, Option<String>>;
}

pub trait ToOptionalString {
    fn to_optional_string(self) -> Option<String>;
}

impl<T: ToString> ToOptionalString for Option<T> {
    fn to_optional_string(self) -> Option<String> {
        self.map(|t| t.to_string())
    }
}

impl ToOptionalString for &str {
    fn to_optional_string(self) -> Option<String> {
        Some(self.to_string())
    }
}

impl ToEnvs for BTreeMap<String, Option<String>> {
    fn to_envs(self) -> BTreeMap<String, Option<String>> {
        self
    }
}

impl<T: ToString, D: ToOptionalString, const Z: usize> ToEnvs for [(T, D); Z] {
    fn to_envs(self) -> BTreeMap<String, Option<String>> {
        BTreeMap::from(self.map(|t| (t.0.to_string(), t.1.to_optional_string())))
    }
}

impl Program {
    pub fn parse<T>(&self, values: T) -> BTreeMap<String, ValueType>
    where
        T: ToEnvs,
    {
        let mut transformer_list: TransformerList = Default::default();

        transformer_list.insert("str", Box::new(StringTransform));
        transformer_list.insert("string", Box::new(StringTransform));
        transformer_list.insert("number", Box::new(NumberTransform));
        transformer_list.insert("bool", Box::new(BooleanTransform));
        transformer_list.insert("boolean", Box::new(BooleanTransform));

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

            let value = Parser::to_parse_variable(&transformer_list, &variable, &envs_values);

            configs.insert(variable.name.to_string(), value);
        }

        configs
    }
}

pub fn create_program<T: ToString>(
    source: T,
    location: Option<T>,
) -> Result<Program, ProgramError> {
    let location_val = location.map(|t| t.to_string());
    ProgramError::create_prepare_context(location_val.clone(), source.to_string(), || {
        Ok(Program {
            location: location_val,
            source: source.to_string(),
            ast: AST::parse(Tokenizer::parse(source.to_string())?)?,
        })
    })
}
