use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Span {
    pub fn substring<T: ToString>(&self, payload: T) -> String {
        payload.to_string()[self.start..self.end].to_string()
    }
}
