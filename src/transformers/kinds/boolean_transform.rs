use crate::transformers::{trait_transformer::Transformer, value_types::ValueType};

pub struct BooleanTransform;

impl Transformer for BooleanTransform {
    fn parse(&self, input_value: String) -> ValueType {
        ValueType::Boolean(match input_value.as_str() {
            "on" | "true" | "1" => true,
            _ => false,
        })
    }
}
