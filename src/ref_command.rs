
pub const NAME: &str = "zix";
pub const VERSION: &str = "v0.0.4";
pub const HELP: &str = r#"USAGE:
    fac <option> <files...>

META OPTIONS:
    [--help, -?]
        print help
    [--version, -v]
        show version of six

DISPLAY OPTIONS:
    [--list -l]
        detailed list format

FILTERING AND SORTING OPTIONS:
    [--all, -a]
        show hidden and 'dot' files

LONG VIEW OPTIONS
    [--headers, -h]
        add a header row to each column
"#;

pub fn help()   {
    println!("{}", &HELP)
}

pub fn version()    {
    println!("{} ({})", &NAME, &VERSION);
    println!("author: Arki (github: @Arkeasz)");
}
