#![feature(try_blocks)]

#[cfg(test)]
mod envuse_test {
    use std::collections::BTreeMap;

    use envuse_parser::create_program;
    use insta::{assert_debug_snapshot, assert_snapshot, assert_yaml_snapshot};

    #[test]
    fn should_catch_program_error() {
        let program = create_program(r#"FOO "biz""#, None).err().unwrap();

        // dbg!(program);

        assert_debug_snapshot!(program);
        assert_snapshot!(program.to_string());
    }

    #[test]
    fn should_catch_program_error_to_js() {
        let error = create_program("\n\n\tFOO 123", Some("/app/.envuse"))
            .err()
            .unwrap();
        assert_snapshot!(error.to_string());

        let error = create_program("\n\n\tFOO 123", None).err().unwrap();
        assert_snapshot!(error.to_string());
    }

    #[test]
    fn should_create_program() {
        let program = create_program(r#"FOO="biz""#, None).unwrap();

        assert_debug_snapshot!(program);
    }

    #[test]
    fn should_use_of_parse_must_not_break_with_different_syntax() {
        let program = create_program(r#"FOO="biz""#, None).unwrap();

        program
            .parse(
                BTreeMap::from([
                    (String::from("AAA"), None),
                    (String::from("FOO"), Some(String::from(""))),
                    (String::from("JUM"), None),
                    (String::from("TAZ"), None),
                ]),
                None,
            )
            .unwrap();

        program
            .parse([("AAA", ""), ("FOO", ""), ("JUM", ""), ("TAZ", "")], None)
            .unwrap();

        program
            .parse(
                [
                    ("AAA", None),
                    ("FOO", Some("")),
                    ("JUM", None),
                    ("TAZ", None),
                ],
                None,
            )
            .unwrap();
    }

    #[test]
    fn should_parse_an_string_value() {
        let program = create_program(r#"FOO="biz""#, None).unwrap();

        assert_debug_snapshot!(program.parse([("FOO", "BAR")], None));
    }

    #[test]
    fn should_parse_an_number_value() {
        let program = create_program(r#"FOO:Number"#, None).unwrap();

        assert_debug_snapshot!(program.parse([("FOO", "30_000")], None));
    }

    #[test]
    fn should_parse_an_boolean_value() {
        let program = create_program(r#"FOO:Boolean"#, None).unwrap();

        assert_debug_snapshot!(program.parse([("FOO", "true")], None));
    }

    #[test]
    fn should_parse_an_null_value() {
        let program = create_program(r#"FOO: String?"#, None).unwrap();

        assert_debug_snapshot!(program.parse([("BAR", "true")], None));
    }

    #[test]
    fn should_read_the_default_values() {
        let program = create_program(r#"FOO: String = "Biz""#, None)
            .map_err(|e| panic!("{}", e.to_string()))
            .unwrap();

        assert_debug_snapshot!(program.parse([("BAR", "true")], None));
    }

    #[test]
    fn should_read_the_default_values_2() {
        let program = create_program(r#"FOO = "Biz""#, None)
            .map_err(|e| panic!("{}", e.to_string()))
            .unwrap();

        assert_debug_snapshot!(program.parse([("BAR", "true")], None));
    }

    #[test]
    fn should_full_sample() {
        let source = r#"
            FOO: String = "val"
            BAR: Number = 1223
            BIZ: Boolean = "true"
            BLI: String?
        "#;

        let program = create_program(source, Some(".envuse"))
            .map_err(|e| panic!("{}", e.to_string()))
            .unwrap();

        let envs: [(&str, &str); 0] = [];

        assert_yaml_snapshot!(program.parse(envs, None).unwrap());
    }

    #[test]
    fn should_catch_error_type_unknown() {
        let source = r###"
            FOO: unknown
        "###;

        let program = create_program(source, Some(".envuse")).unwrap();
        let error = program.parse(None, None).unwrap_err();

        assert_snapshot!(error.to_string());
        assert_debug_snapshot!(error);
    }

    #[test]
    fn should_parse_custom_types() {
        let source = r###"
            FOO: FIZ
        "###;

        let program = create_program(source, Some(".envuse")).unwrap();

        let custom_transformers = ["FIZ"];
        let values = [("FOO", "bar")];

        let parsed = program.parse(values, custom_transformers).unwrap();

        assert_debug_snapshot!(parsed);
    }
}
