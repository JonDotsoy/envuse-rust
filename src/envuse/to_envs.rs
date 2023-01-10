use std::collections::BTreeMap;

use super::to_optional_string::ToOptionalString;

pub trait ToEnvs {
    fn to_envs(self) -> BTreeMap<String, Option<String>>;
}

impl ToEnvs for BTreeMap<String, Option<String>> {
    fn to_envs(self) -> BTreeMap<String, Option<String>> {
        self
    }
}

impl<T: ToString, D: ToOptionalString, const Z: usize> ToEnvs for [(T, D); Z] {
    fn to_envs(self) -> BTreeMap<String, Option<String>> {
        BTreeMap::from(self.map(|t| (t.0.to_string(), t.1.to_optional_string())))
    }
}

impl ToEnvs for Option<BTreeMap<String, Option<String>>> {
    fn to_envs(self) -> BTreeMap<String, Option<String>> {
        BTreeMap::new()
    }
}
