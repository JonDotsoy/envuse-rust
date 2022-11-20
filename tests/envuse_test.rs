#[cfg(test)]
mod envuse_test {
    use envuse_parser::envuse::{Envuse, ParseOptions};
    use std::collections::BTreeMap;

    #[test]
    fn should_compile() {
        let result = Envuse::compile(ParseOptions {
            source: String::from(r#"
                FOO:String
                PORT:Number=3000
                MONGO_DB:String<Format="URL">
                FORCE_SSL:Boolean?
            "#),
            environment_values: BTreeMap::from([
                ("FOO", "BIZ"),
                ("PORT", "5000"),
            ]),
        });
    }

    fn should_create_dts_compiler() {}
}
