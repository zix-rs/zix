use std::io::{self, Write};

use zix_core::entry::options::Opti;

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
    let mut out = String::from("USAGE: zx <option> <files...>\n");
    out += "\nZIX OPTIONS:\n";

    let options = [
        Opti::All,
        Opti::Grid,
        Opti::Headers,
        Opti::Help,
        Opti::Icons,
        Opti::List,
        Opti::Tree,
        Opti::Version
    ];

    for option in &options {
        out += &format!("  {}, {}\t {}\n", option.as_flag(), option.as_short_flag().unwrap_or(""), option.description());
    }

    let _ = io::stdout().write_all(out.as_bytes());
}

pub fn version()    {
    println!("{} ({})", &NAME, &VERSION);
    println!("author: Arki (github: @Arkeasz)");
}
