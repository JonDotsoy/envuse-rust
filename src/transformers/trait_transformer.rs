use super::value_types::ValueType;

pub trait Transformer {
    fn parse(&self, input_value: String) -> ValueType;
}
