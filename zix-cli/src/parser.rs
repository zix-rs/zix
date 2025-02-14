use std::env;
use zix_core::entry::create::FilterOptions;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Opti   {
    All,
    List,
    Help,
    Version,
    Headers,
    Icons,
    Tree,
    Grid
}


impl FilterOptions for Opti {
    fn should_include_hidden(&self) -> bool {
        matches!(self, Opti::All)
    }

    fn should_show_icons(&self) -> bool {
        matches!(self, Opti::Icons)
    }

    fn should_use_tree_view(&self) -> bool {
        matches!(self, Opti::Tree)
    }

    fn should_use_list_view(&self) -> bool {
        matches!(self, Opti::List)
    }

    fn should_use_grid_view(&self) -> bool {
        matches!(self, Opti::Grid)
    }
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
