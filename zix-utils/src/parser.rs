use std::env;

pub fn parser(
    has_sub_commands: bool,
    value_default: &str,
    option_default: &str,
    command_default: &str
) -> Option<(String, Vec<String>, Vec<String>)>  {
    let args = env::args().skip(1).collect::<Vec<String>>();
    let v_default = value_default.to_string();
    let o_default = option_default.to_string();
    let c_default = command_default.to_string();

    let mut options: Vec<String> = Vec::new();
    let mut command = String::new();
    let mut values: Vec<String> = Vec::new();

    let mut iter = args.iter();
    if has_sub_commands {
        if let Some(first) = iter.next() {
            command = first.clone();
        }
    }


    for arg in iter {
        if arg.starts_with("--") {
            options.push(arg.clone());
        } else if arg.starts_with('-') && arg.len() > 1 {
            options.extend(arg[1..].chars().map(|ch| format!("-{}", ch)));
        } else {
            values.push(arg.clone());
        }
    }

    if values.is_empty() {
        values.push(v_default);
    }

    if options.is_empty() {
        options.push(o_default);
    }

    if command.is_empty() {
        command = c_default;
    }

    Some((command, options, values))
}
