use crate::transformers::{trait_transformer::Transformer, value_types::ValueType};

pub struct NumberTransform;

impl Transformer for NumberTransform {
    fn parse(&self, type_input: String,input_value: String) -> ValueType {
        ValueType::Number(input_value.replace("_", "").parse::<u32>().unwrap())
    }
}
