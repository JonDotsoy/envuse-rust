use crate::transformers::{trait_transformer::Transformer, value_types::ValueType};

pub struct CustomTransform;

impl Transformer for CustomTransform {
    fn parse(&self, type_input: String, input_value: String) -> ValueType {
        ValueType::Custom(type_input, input_value)
    }
}
