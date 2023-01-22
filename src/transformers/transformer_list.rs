use super::trait_transformer::Transformer;
use std::collections::HashMap;


type TransformerType = Box<dyn Transformer>;

pub struct TransformerList {
    transformers: HashMap<String, TransformerType>,
}

impl TransformerList {
    pub fn new() -> Self {
        Self {
            transformers: Default::default(),
        }
    }

    pub fn insert<T: ToString>(&mut self, key: T, transformer: TransformerType) {
        self.transformers.insert(key.to_string(), transformer);
    }

    pub fn get<T: ToString>(&self, transform_type: T) -> Option<&Box<dyn Transformer>> {
        self.transformers.get(&transform_type.to_string())
    }
}

impl Default for TransformerList {
    fn default() -> Self {
        Self::new()
    }
}
