#[cfg(test)]
mod tokenizer {
    use envuse_parser::{
        parser::tokenizer::{Token, Tokenizer},
        syntax_error::SyntaxError,
    };
    use insta::assert_debug_snapshot;
    use std::dbg;

    #[test]
    fn tokenizer_comment() {
        let payload = "# abc";
        let tokens = Tokenizer::parse(payload).unwrap();
        assert_debug_snapshot!(tokens);
    }

    #[test]
    fn tokenizer_comment_and_newline_1() {
        let payload = "# abc\n";
        let tokens = Tokenizer::parse(payload).unwrap();
        assert_debug_snapshot!(tokens);
    }

    #[test]
    fn tokenizer_comment_and_newline_2() {
        let payload = "# abc\n# defg\n";
        let tokens = Tokenizer::parse(payload).unwrap();
        assert_debug_snapshot!(tokens);
    }

    #[test]
    fn tokenizer_keyword() {
        let payload = "ABC123_32";
        let tokens = Tokenizer::parse(payload).unwrap();
        assert_debug_snapshot!(tokens);
    }

    #[test]
    fn tokenizer_keyword_and_newline() {
        let payload = "ABC123_32\n";
        let tokens = Tokenizer::parse(payload).unwrap();
        assert_debug_snapshot!(tokens);
    }

    #[test]
    fn tokenizer_colon() {
        let payload = ":";
        let tokens = Tokenizer::parse(payload).unwrap();
        assert_debug_snapshot!(tokens);
    }

    #[test]
    fn tokenizer_string() {
        let payload = r#""hola""#;
        let tokens = Tokenizer::parse(payload).unwrap();
        assert_debug_snapshot!(tokens);
    }

    #[test]
    fn tokenizer_string_with_scape() {
        let payload = r#""ho\"la""#;
        let tokens = Tokenizer::parse(payload).unwrap();
        assert_debug_snapshot!(tokens);
    }

    #[test]
    fn tokenizer_variable() {
        let payload = "FOO:BAZ";
        let tokens = Tokenizer::parse(payload).unwrap();
        assert_debug_snapshot!(tokens);
    }

    #[test]
    fn tokenizer_comment_and_variable() {
        let payload = "# comment\nFOO:BAZ";
        let tokens = Tokenizer::parse(payload).unwrap();
        assert_debug_snapshot!(tokens);
    }

    #[test]
    fn tokenizer_variable_with_default_value() {
        let payload = r#"FOO:BAZ="abc""#;
        let tokens = Tokenizer::parse(payload).unwrap();
        assert_debug_snapshot!(tokens);
    }

    #[test]
    fn tokenizer_number_1() {
        let payload = r#"12"#;
        let tokens = Tokenizer::parse(payload).unwrap();
        assert_debug_snapshot!(tokens);
    }

    #[test]
    fn tokenizer_number_2() {
        let payload = r#"12_123_456.123_456"#;
        let tokens = Tokenizer::parse(payload).unwrap();
        dbg!(tokens);
    }

    #[test]
    fn tokenizer_number_3() {
        Tokenizer::parse("123").unwrap();
        Tokenizer::parse("1_23").unwrap();
        Tokenizer::parse("1_23.23").unwrap();

        assert_eq!(
            Tokenizer::parse("12__23").unwrap_err().to_string(),
            "Only one underscore is allowed as numeric separator"
        );

        assert_eq!(
            Tokenizer::parse("12__23").unwrap_err().to_string(),
            "Only one underscore is allowed as numeric separator",
        );
        assert_eq!(
            Tokenizer::parse("12_").unwrap_err().to_string(),
            "Only one underscore is allowed as numeric separator",
        );
        assert_eq!(
            Tokenizer::parse("12_3._").unwrap_err().to_string(),
            "Invalid or unexpected token"
        );
        assert_eq!(
            Tokenizer::parse("12_3._").unwrap_err().to_string(),
            "Invalid or unexpected token"
        );
        assert_eq!(
            Tokenizer::parse("12_3.3_3.1").unwrap_err().to_string(),
            "Unexpected token"
        );
    }

    #[test]
    fn tokenizer_spaces() {
        let tokens = Tokenizer::parse("foo : string = asd\nvar : int = \"234\"").unwrap();
        assert_debug_snapshot!(tokens);
    }

    #[test]
    fn tokenizer_type_with_optional_variable() {
        let tokens = Tokenizer::parse("String?").unwrap();
        assert_debug_snapshot!(tokens);
    }

    #[test]
    fn tokenizer_type_with_options() {
        let tokens = Tokenizer::parse("String<Min=2 Max=10>").unwrap();
        assert_debug_snapshot!(tokens);
    }

    #[test]
    fn tokenizer_unexpected_token_error() {
        let err = Tokenizer::parse("/").err();
        assert_debug_snapshot!(err);
    }
}
