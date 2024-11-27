
pub const NAME: &str = "zit";
pub const VERSION: &str = "v0.0.3";
pub const HELP: &str = r#"Usage:
    fac <option> <directory>

    Options:
        [--help, -h]
            print help
        [--version, -v]
            print command version
        [--list -l]
            detailed list format
        [--all, -a]
            include hidden files
"#;

pub fn help()   {
    println!("{}", &HELP)
}

pub fn version()    {
    println!("{} ({})", &NAME, &VERSION);
    println!("author: Arki (github: @Arkeasz)");
}
