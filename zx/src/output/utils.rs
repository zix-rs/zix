use regex;


pub fn strip_ansi_codes(input: &str) -> String {
    let ansi_regex = regex::Regex::new(r"\x1b\[[0-9;]*m").unwrap();
    ansi_regex.replace_all(input, "").to_string()
}

pub fn adjust_terminal_width(term_width: u16) -> usize  {
    let adjusted_width = if term_width >= 80 {
        term_width.saturating_sub(20)
    } else {
        term_width
    };
    adjusted_width.max(1).into()
}
