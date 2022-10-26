#[cfg(test)]
mod tokenizer {
    use envuse_parser::parser::tokenizer::Tokenizer;
    use insta::assert_debug_snapshot;
    use std::dbg;

    #[test]
    fn tokenizer_comment() {
        let payload = "# abc";
        let tokens = Tokenizer::parse(payload);
        assert_debug_snapshot!(tokens);
    }

    #[test]
    fn tokenizer_comment_and_newline_1() {
        let payload = "# abc\n";
        let tokens = Tokenizer::parse(payload);
        assert_debug_snapshot!(tokens);
    }

    #[test]
    fn tokenizer_comment_and_newline_2() {
        let payload = "# abc\n# defg\n";
        let tokens = Tokenizer::parse(payload);
        assert_debug_snapshot!(tokens);
    }
}
