#[cfg(test)]
mod ast_test {
    use envuse_parser::parser::span::Span;
    use envuse_parser::parser::tokenizer::Tokenizer;

    use envuse_parser::{parser::ast::AST, utils::display_syntax::DisplaySyntax};
    use insta::{assert_debug_snapshot, assert_snapshot, assert_yaml_snapshot};

    #[test]
    fn parse_executable() {
        let payload = "#!/bin/sh";
        let tokens = Tokenizer::parse(payload).unwrap();
        assert_debug_snapshot!(AST::parse(tokens).unwrap());
    }

    #[test]
    fn parse_comment() {
        let payload = "# I'm comment";
        let tokens = Tokenizer::parse(payload).unwrap();
        assert_debug_snapshot!(AST::parse(tokens).unwrap());
    }

    #[test]
    fn parse_comment_block() {
        let payload = "# I'm comment\n# Second line comment";
        let tokens = Tokenizer::parse(payload).unwrap();
        assert_debug_snapshot!(AST::parse(tokens).unwrap());
    }

    #[test]
    fn parse_comment_multiple_block() {
        let payload = "# I'm comment\n\n# Second line comment";
        let tokens = Tokenizer::parse(payload).unwrap();
        assert_debug_snapshot!(AST::parse(tokens).unwrap());
    }

    #[test]
    fn parse_comment_multiple_lines() {
        let payload = "# I'm comment\n    # Second line comment";
        let tokens = Tokenizer::parse(payload).unwrap();
        assert_debug_snapshot!(AST::parse(tokens).unwrap());
    }

    #[test]
    fn parse_comment_multiple_block_2() {
        let payload = "# I'm comment\n\n     # Second line comment";
        let tokens = Tokenizer::parse(payload).unwrap();
        assert_debug_snapshot!(AST::parse(tokens).unwrap());
    }

    #[test]
    fn parse_variables_name() {
        let payload = "
            foo
        ";
        let tokens = Tokenizer::parse(payload).unwrap();
        assert_debug_snapshot!(AST::parse(tokens).unwrap());
    }

    #[test]
    fn parse_variables_with_comment() {
        let payload = "
            # comment
            foo
        ";
        let tokens = Tokenizer::parse(payload).unwrap();
        assert_debug_snapshot!(AST::parse(tokens).unwrap());
    }

    #[test]
    fn parse_variables_with_type() {
        let payload = "
            foo: string
        ";
        let tokens = Tokenizer::parse(payload).unwrap();
        assert_debug_snapshot!(AST::parse(tokens).unwrap());
    }

    #[test]
    fn parse_variables_as_nullable() {
        let payload = "
            FOO: String<Foo Biz> ?
        ";
        let tokens = Tokenizer::parse(payload).unwrap();
        assert_debug_snapshot!(AST::parse(tokens).unwrap());
    }

    #[test]
    fn parse_variable_with_default_value() {
        let payload = r#"foo = "abc""#;
        let tokens = Tokenizer::parse(payload).unwrap();
        assert_debug_snapshot!(AST::parse(tokens).unwrap());
    }

    #[test]
    fn parse_variable_with_default_value_number() {
        let payload = r#"port: number = 3000"#;
        let tokens = Tokenizer::parse(payload).unwrap();
        assert_debug_snapshot!(AST::parse(tokens).unwrap());
    }

    #[test]
    fn parse_variable() {
        let payload = r#"
            # comment
            foo: string = "abc"
        "#;
        let tokens = Tokenizer::parse(payload).unwrap();
        assert_debug_snapshot!(AST::parse(tokens).unwrap());
    }

    #[test]
    fn parse_sample_document() {
        let payload = r#"
            ## Document comment
            ## second line

            # comment
            foo: string = "abc"
            bax: number = "123"
            port: number = 3_000
        "#;
        let tokens = Tokenizer::parse(payload).unwrap();
        assert_debug_snapshot!(AST::parse(tokens).unwrap());
    }

    #[test]
    fn parse_unexpected_token() {
        let payload = r#"

            _$3

        "#;

        let err = Tokenizer::parse(payload).err();
        assert_debug_snapshot!(err);
    }

    // fn s<T: ToString, F: FnOnce() -> Result<(), SyntaxError>>(payload: &T, f: F) {
    //     match f() {
    //         Ok(_) => {}
    //         Err(err) => {
    //             // dbg!(err.debug_payload(&*payload))
    //         }
    //     }
    // }

    #[test]
    fn should_decorate_syntax_error() {
        let payload = "#!/bin/envuse\nFOO: String<Max=500>\nBIZ: String<Max=500 Min=2>\n";

        assert_snapshot!(
            DisplaySyntax::new("SyntaxError: fail", Span { start: 14, end: 17 })
                .debug_payload(&payload)
        );
    }

    #[test]
    fn should_decorate_syntax_error_2() {
        let payload =
            "#!/bin/envuse\n# Inline comment\nFOO: String<Max=500>\nBIZ: String<Max=500 Min=2>\nPORT: Number\n";

        let err = DisplaySyntax::new("SyntaxError: fail", Span { start: 36, end: 83 });

        // dbg!(err.span.substring(&payload));

        assert_snapshot!(err.debug_payload(&payload));
    }

    #[test]
    fn parse_sample_document_with_type_configurable() {
        let payload = r#"
            FOO : String<Max=500>
            BIZ : String<Max=500 Min=2>
        "#;

        let tokens = Tokenizer::parse(payload).unwrap();
        assert_yaml_snapshot!(AST::parse(tokens).unwrap());
        let tokens = Tokenizer::parse(payload).unwrap();
        assert_debug_snapshot!(AST::parse(tokens).unwrap());
    }

    #[test]
    fn parse_sample_document_with_type_configurable_binary_option() {
        let payload = r#"
            FOO : String<Max=500 Sensitive>
            BIZ : String<Max=500 Min=2>
        "#;

        let tokens = Tokenizer::parse(payload).unwrap();
        assert_yaml_snapshot!(AST::parse(tokens).unwrap());
        let tokens = Tokenizer::parse(payload).unwrap();
        assert_debug_snapshot!(AST::parse(tokens).unwrap());
    }

    #[test]
    fn parse_sample_document_with_type_configurable_multiline() {
        let payload = r#"
            FOO : String<
                    Max = 500
                    Min = 2
                  >
        "#;

        let tokens = Tokenizer::parse(payload).unwrap();
        assert_debug_snapshot!(AST::parse(tokens).unwrap());
    }
}
