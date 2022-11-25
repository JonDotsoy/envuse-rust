#[derive(Debug)]
pub enum ValueType {
    Null,
    Boolean(bool),
    Number(u32),
    String(String),
}
