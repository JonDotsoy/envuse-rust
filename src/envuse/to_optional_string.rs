pub trait ToOptionalString {
    fn to_optional_string(self) -> Option<String>;
}

impl<T: ToString> ToOptionalString for Option<T> {
    fn to_optional_string(self) -> Option<String> {
        self.map(|t| t.to_string())
    }
}

impl ToOptionalString for &str {
    fn to_optional_string(self) -> Option<String> {
        Some(self.to_string())
    }
}
