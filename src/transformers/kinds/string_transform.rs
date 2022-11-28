use crate::transformers::{trait_transformer::Transformer, value_types::ValueType};

pub struct StringTransform;

impl Transformer for StringTransform {
    fn parse(&self, input_value: String) -> ValueType {
        ValueType::String(input_value)
    }
}
