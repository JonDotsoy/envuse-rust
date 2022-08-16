use serde::Serialize;

use super::super::kind::Kind;

#[derive(Debug, Clone, Serialize)]
pub struct Literal(pub String);

impl From<Literal> for Kind {
    fn from(v: Literal) -> Self {
        Kind::Literal(v)
    }
}
