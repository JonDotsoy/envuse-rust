#[cfg(test)]
mod program_error_test {
    use envuse_parser::errors::program_error::ProgramError;

    #[test]
    fn should_transform_error() {
        let _err = ProgramError {
            cause: None,
            location: None,
            message: "".to_string(),
            source: "".to_string(),
            span: None,
        };

        // dbg!(err.to_string());
    }
}
