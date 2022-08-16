use serde::Serialize;
use super::kind::Kind;
use super::token::Token;

#[derive(Debug, Clone, Serialize)]
pub struct Node(pub Token, pub Kind);

impl Node {
    // experimental
    pub fn to_string(self, payload: &[u8]) -> &[u8] {
        &payload[self.0.span.range.start..self.0.span.range.end]
    }

    pub fn to_token(self) -> Token {
        self.0
    }

    pub fn to_kind(self) -> Kind {
        self.1
    }
}
