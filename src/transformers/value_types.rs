use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum ValueType {
    Null,
    Boolean(bool),
    Number(u32),
    String(String),
    Custom(String, String),
}
