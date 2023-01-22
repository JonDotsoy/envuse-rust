pub trait ToCustomTransformers {
    fn to_vec(self) -> Vec<String>;
}

impl ToCustomTransformers for Option<String> {
    fn to_vec(self) -> Vec<String> {
        vec![]
    }
}

impl ToCustomTransformers for Vec<&str> {
    fn to_vec(self) -> Vec<String> {
        self.iter().map(|s| s.to_string()).collect()
    }
}

impl<T: ToString, const Z: usize> ToCustomTransformers for [T; Z] {
    fn to_vec(self) -> Vec<String> {
        self.iter().map(|s| s.to_string()).collect()
    }
}
