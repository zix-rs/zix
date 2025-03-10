use std::io::{self, Write};

pub const NAME: &str = "zx";
pub const VERSION: &str = "v0.0.8";
pub const HELP: &str = r#"USAGE:
    zx <option> <files...>

META OPTIONS:
    [--help, -?]
        print help
    [--version, -v]
        show version of zx

DISPLAY OPTIONS:
    [--list -l]
        detailed list format
    [--tree, -t]
        recurse into directories as a tree
    [--grid, -g]
        show grid format

FILTERING AND SORTING OPTIONS:
    [--all, -a]
        show hidden and 'dot' files

LONG VIEW OPTIONS
    [--headers, -h]
        add a header row to each column
"#;

pub fn help() {
    let _ = io::stdout().write_all(HELP.as_bytes());
}

pub fn version()    {
    println!("{} ({})", &NAME, &VERSION);
    println!("author: Arki (github: @Arkeasz)");
}
