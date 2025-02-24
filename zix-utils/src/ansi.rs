pub fn strip_ansi_codes(input: &str) -> String {
    let ansi_regex = regex::Regex::new(r"\x1b\[[0-9;]*m").unwrap();
    ansi_regex.replace_all(input, "").to_string()
}
