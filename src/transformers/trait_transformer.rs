use super::value_types::ValueType;

pub trait Transformer {
    fn parse(&self, type_input: String, input_value: String) -> ValueType;
}
