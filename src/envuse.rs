#[cfg(feature = "with-js")]
use js_sys::SyntaxError as JSSyntaxError;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
#[cfg(feature = "with-js")]
use wasm_bindgen::JsValue;

#[derive(Debug)]
pub enum ParsedValue {
    String(String),
    Number(u32),
    Boolean(bool),
    Null,
}

trait Parser {
    fn to_parsed_value(
        name: &String,
        variable_type: &Option<String>,
        options_variable_type: &Option<BTreeMap<String, Option<Expression>>>,
        default_value: &Box<Option<Expression>>,
        nullable: &bool,
        intent_value: &Option<String>,
    ) -> ParsedValue;
}

impl Parser for ParsedValue {
    fn to_parsed_value(
        name: &String,
        variable_type: &Option<String>,
        _options_variable_type: &Option<BTreeMap<String, Option<Expression>>>,
        default_value: &Box<Option<Expression>>,
        nullable: &bool,
        intent_value: &Option<String>,
    ) -> Self {
        let transform_type = variable_type.clone().unwrap_or("String".to_string());

        if nullable == &true && intent_value.is_none() && default_value.is_none() {
            return ParsedValue::Null;
        }

        let proposal_value: String = if let Some(v) = intent_value {
            v.clone()
        } else if let Some(Expression::DefaultValue { value, .. }) = default_value.as_ref() {
            value.clone()
        } else {
            panic!("{}", format!("Cannot found env {}", &name));
        };

        match transform_type.to_lowercase().as_str() {
            "string" => ParsedValue::String(proposal_value),
            "number" => {
                ParsedValue::Number(proposal_value.replace("_", "").parse::<u32>().unwrap())
            }
            "boolean" => match proposal_value.to_lowercase().as_str() {
                "on" | "true" | "1" => ParsedValue::Boolean(true),
                _ => ParsedValue::Boolean(false),
            },
            _ => panic!("Type {} is not valid", transform_type),
        }
    }
}

use crate::{
    parser::{
        ast::{Expression, AST},
        span::Span,
        tokenizer::Tokenizer,
    },
    syntax_error::{DebugOptions, SyntaxError},
};

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
            Ok(r) => Ok(r),
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
    pub fn parse<T: ToEnvs>(&self, values: T) -> BTreeMap<String, ParsedValue> {
        let envs_values = values.to_envs();

        let expressions = match &self.ast {
            Expression::Document { elements, .. } => elements,
            _ => panic!("AST Parser error, the ast expressions is not Expression::Document"),
        };

        let mut configs: BTreeMap<String, ParsedValue> = BTreeMap::new();

        for expression in expressions {
            let (key, value) = match expression {
                Expression::Variable {
                    span,
                    comment: _,
                    name,
                    variable_type,
                    options_variable_type,
                    default_value,
                    nullable,
                } => (
                    name,
                    ParsedValue::to_parsed_value(
                        name,
                        variable_type,
                        options_variable_type,
                        default_value,
                        nullable,
                        envs_values.get(name).unwrap_or(&None), // envs_values.get(&variable_type),
                    ),
                ),
                _ => todo!("Expression is not supported"),
            };

            configs.insert(key.to_string(), value);
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
