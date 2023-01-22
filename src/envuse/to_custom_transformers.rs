use std::{ptr::NonNull, vec};

pub trait ToCustomTransformers {
    fn to_vec(self) -> Vec<String>;
}

impl<T: ToString> ToCustomTransformers for Option<Vec<T>> {
    fn to_vec(self) -> Vec<String> {
        match self {
            Some(a) => a.iter().map(|s| s.to_string()).collect(),
            None => vec![],
        }
    }
}

impl<T: ToString> ToCustomTransformers for Vec<T> {
    fn to_vec(self) -> Vec<String> {
        self.iter().map(|s| s.to_string()).collect()
    }
}

impl<T: ToString, const Z: usize> ToCustomTransformers for [T; Z] {
    fn to_vec(self) -> Vec<String> {
        self.iter().map(|s| s.to_string()).collect()
    }
}
