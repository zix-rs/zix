pub mod meta;
pub mod com;
pub mod out;
use com::ZixManagerCommands;
use com::ZixManagerCommands::{
    Init,
    List,
    Install,
    Update,
    Help,
    Version,
    Neither
};
use zix_utils::parser::parser;

fn main() {
    if let Some((
        co,
        _op,
        _val
    )) = parser(
        true,
        "",
        "",
        ""
    ) {
        match co.as_str()   {
            "init"          => Init.run(),
            "list" | "l"    => List.run(),
            "install" | "i" => Install.run(),
            "help" | "h"    => Help.run(),
            "update"        => Update.run(),
            "version" | "v" => Version.run(),
            _               => Neither.run(),
        }
    }
}
