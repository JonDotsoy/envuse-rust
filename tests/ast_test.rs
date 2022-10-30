#[cfg(test)]
mod ast_test {
    use envuse_parser::parser::{ast::AST};
    use envuse_parser::parser::tokenizer::Tokenizer;
    use insta::assert_debug_snapshot;

    #[test]
    fn parse_executable() {
        let payload = "#!/bin/sh";
        let tokens = Tokenizer::parse(payload);
        assert_debug_snapshot!(AST::parse(tokens));
    }

    #[test]
    fn parse_comment() {
        let payload = "# I'm comment";
        let tokens = Tokenizer::parse(payload);
        assert_debug_snapshot!(AST::parse(tokens));
    }

    #[test]
    fn parse_comment_block() {
        let payload = "# I'm comment\n# Second line comment";
        let tokens = Tokenizer::parse(payload);
        assert_debug_snapshot!(AST::parse(tokens));
    }

    #[test]
    fn parse_comment_multiple_block() {
        let payload = "# I'm comment\n\n# Second line comment";
        let tokens = Tokenizer::parse(payload);
        assert_debug_snapshot!(AST::parse(tokens));
    }

    #[test]
    fn parse_comment_multiple_lines() {
        let payload = "# I'm comment\n    # Second line comment";
        let tokens = Tokenizer::parse(payload);
        assert_debug_snapshot!(AST::parse(tokens));
    }

    #[test]
    fn parse_comment_multiple_block_2() {
        let payload = "# I'm comment\n\n     # Second line comment";
        let tokens = Tokenizer::parse(payload);
        assert_debug_snapshot!(AST::parse(tokens));
    }

    #[test]
    fn parse_variables_name() {
        let payload = "
            foo
        ";
        let tokens = Tokenizer::parse(payload);
        assert_debug_snapshot!(AST::parse(tokens));
    }

    #[test]
    fn parse_variables_with_comment() {
        let payload = "
            # comment
            foo
        ";
        let tokens = Tokenizer::parse(payload);
        assert_debug_snapshot!(AST::parse(tokens));
    }

    #[test]
    fn parse_variables_with_type() {
        let payload = "
            foo: string
        ";
        let tokens = Tokenizer::parse(payload);
        assert_debug_snapshot!(AST::parse(tokens));
    }

    #[test]
    fn parse_variable_with_default_value() {
        let payload = r#"foo = "abc""#;
        let tokens = Tokenizer::parse(payload);
        assert_debug_snapshot!(AST::parse(tokens));
    }

    #[test]
    fn parse_variable_with_default_value_number() {
        let payload = r#"port: number = 3000"#;
        let tokens = Tokenizer::parse(payload);
        assert_debug_snapshot!(AST::parse(tokens));
    }

    #[test]
    fn parse_variable() {
        let payload = r#"
            # comment
            foo: string = "abc"
        "#;
        let tokens = Tokenizer::parse(payload);
        assert_debug_snapshot!(AST::parse(tokens));
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
        let tokens = Tokenizer::parse(payload);
        assert_debug_snapshot!(AST::parse(tokens));
    }
}
