pub trait ToCustomTransformers {
    fn to_vec(self) -> Vec<String>;
}

impl ToCustomTransformers for Option<String> {
    fn to_vec(self) -> Vec<String> {
        vec![]
    }
}
