pub mod meta;
use zix_utils::parser::parser;

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

fn main() {
    if let Some((
        co,
        _op,
        _val
    )) = parser(true, "", "", "") {
        match co.as_str()   {
            "init" => println!("init"),
            "list" | "l" => println!("list"),
            "install" | "i" => println!("install"),
            "help" | "h" => meta::help(),
            "version" | "v" => meta::version(),
            _ => {
                println!(
                    "That's not a valid option\nType 'zix help' for more information."
                );
            }
        }
    }
}
