use std::env;
#[derive(Clone, Copy, PartialEq, Debug)]

pub enum Opti   {
    All,
    List,
    Help,
    Version,
    Headers,
    Icons,
    Tree
}

pub fn parse() -> (Vec<String>, Vec<String>)  {
    let args = env::args().skip(1);

    let mut options: Vec<String> = Vec::new();
    let mut values: Vec<String> = Vec::new();

    for arg in args {
        if arg.starts_with("--") {
            options.push(arg);
        } else if arg.starts_with('-') && arg.len() > 1 {
            options.extend(arg[1..].chars().map(|ch| format!("-{}", ch)));
        } else {
            values.push(arg);
        }
    }

    if values.is_empty() {
        values.push(".".to_string());
    }

    (options, values)
}
