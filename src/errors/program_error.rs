use super::super::envuse::program::Program;
use crate::parser::span::Span;
use crate::syntax_error::DebugOptions;
use crate::syntax_error::SyntaxError;
use std::error::Error;
use std::fmt;

// #[cfg(feature = "with-js")]
use js_sys::SyntaxError as JSSyntaxError;
use wasm_bindgen::JsValue;

#[derive(Debug)]
pub struct ProgramError {
    pub message: String,
    pub span: Option<Span>,
    pub source: String,
    pub location: Option<String>,
    pub cause: Option<SyntaxError>,
}

impl ProgramError {
    pub fn get_message(&self) -> String {
        let mut debug_options = DebugOptions::new();
        debug_options.location = self.location.clone();

        self.cause
            .as_ref()
            .unwrap()
            .debug_payload_configurable(&self.source, &debug_options)
    }
}

impl fmt::Display for ProgramError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.get_message().to_string())
    }
}

impl std::error::Error for ProgramError {}
