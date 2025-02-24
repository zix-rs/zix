
pub const NAME: &str = "zix";
pub const VERSION: &str = "v0.0.1";
pub const HELP: &str = r#"ZIX - A custom file manager and utility tool

USAGE:
    zix <command> [options]

META OPTIONS:
    help, h      Show this help message
    version, v   Show the current version of Zix

COMMANDS:
    init         Initialize a new configuration or setup
    install, i   Install a specific version of ZX
    update, u    Update ZX and Zix to the latest version
    list, l      Show installed versions of Zix and ZX

EXAMPLES:
    zix help
    zix version
    zix install 0.0.5
    zix list
"#;


pub fn help()   {
    println!("{}", &HELP)
}

pub fn version()    {
    println!("{} ({})", &NAME, &VERSION);
    println!("author: Arki (github: @Arkeasz)");
}
