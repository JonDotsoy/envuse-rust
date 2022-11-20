use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Span {
    pub fn substring<T: ToString>(&self, payload: T) -> String {
        payload.to_string()[self.start..self.end].to_string()
    }
}
