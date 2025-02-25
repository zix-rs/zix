#[cfg(test)]
mod tests {
    use zix_utils::parser::parser;
    use std::env;

    #[test]
    fn test_parser_with_defaults() {
        let args = vec!["program".to_string()];
        env::set_var("RUST_TEST_ARGS", args.join(" "));
        let result = parser(true, "default_value", "--default_option", "default_command");
        assert_eq!(
            result,
            Some((
                "default_command".to_string(),
                vec!["--default_option".to_string()],
                vec!["default_value".to_string()]
            ))
        );
    }


}
