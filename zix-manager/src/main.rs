pub mod metadata;
use std::env;

use zix_utils::parser::parse;

enum ZixManagerCommands   {
    Help,
    Update,
    Init,
    Install,
    Version,
    List
}

struct ZixManager   {
    options: Vec<ZixManagerCommands>
}

impl ZixManager {
    pub fn new() -> ZixManager {
        ZixManager  {
            options: Vec::new()
        }
    }
}

fn parser() -> Option<(String, Vec<String>, Vec<String>)>  {
    let mut args = env::args().skip(1).collect::<Vec<String>>();

    if args.is_empty() {
        args.push("help".to_string());
    }

    let mut options: Vec<String> = Vec::new();
    let mut command = String::new();
    let mut values: Vec<String> = Vec::new();

    let mut iter = args.iter();


    if let Some(first) = iter.next() {
        if !first.starts_with('-') {
            command = first.clone();
        } else {
            options.push(first.clone());
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

    Some((command, options, values))
}
fn main() {
    if let Some((co, op, val)) = parser() {
        match co.as_str()   {
            "init" => println!("init"),
            "list" | "l" => println!("list"),
            "install" | "i" => println!("install"),
            "help" | "h" => metadata::help(),
            "version" | "v" => metadata::version(),
            _ => {
                println!(
                    "That's not a valid option\nType 'zx --help' for more information."
                );
            }
        }
    }
}
