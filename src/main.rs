pub mod entry;
pub mod app;
pub mod ref_command;
pub mod parser;
pub mod window;
pub mod output;
use app::App;
use parser::Opti;



fn main() {
    if let Some(app) = App::init() {
        let items = app.entries;
        let ops = app.options;

        for dir_entries in &items {
            let mut vect_entry_name: Vec<String> = Vec::new();

            for entry in dir_entries {
                vect_entry_name.push(entry.name.clone());
            }

            if ops.contains(&Opti::Tree) {
                output::tree::base(&dir_entries, ops.clone());
            } else if ops.contains(&Opti::List) {
                output::list::base(&dir_entries, ops.clone());
            } else {
                output::grid::base(vect_entry_name, &dir_entries);
            }
        }
    }
}
