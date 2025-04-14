use crate::out::{
    init,
    list,
    help
};

/**
 * .zix
 * ├─ plugins
 * │   ├─ zix_better_grid.dll
 * │   └─ zix_hello.dll
 * └─ .config.toml
**/

#[allow(dead_code)]
pub enum ZixManagerCommands   {
    Help,
    Update,
    Init,
    Install,
    Version,
    List,
    Neither
}

impl ZixManagerCommands {
    pub fn run(&self) {
        match self {
            ZixManagerCommands::Init    => init(),
            ZixManagerCommands::List    => list(),
            ZixManagerCommands::Help    => help(),
            ZixManagerCommands::Version => println!("version"),
            ZixManagerCommands::Install => println!("install"),
            ZixManagerCommands::Update  => println!("update"),
            ZixManagerCommands::Neither => println!("That's not a valid option\nType 'zix help' for more information.")
        }
    }
}
