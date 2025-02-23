pub mod app;
pub mod output;
pub mod ref_command;
pub mod window;

use app::App;
use zix_core::entry::create::Opti;
fn main() {
    if let Some(app) = App::init() {
        let mut items = app.entries;
        let ops = app.options;

        for dir_entries in items.iter_mut() {
            let mut vect_entry_name: Vec<String> = Vec::new();

            for entry in dir_entries.iter() {
                vect_entry_name.push(entry.name.clone());
            }

            if ops.contains(&Opti::Tree) {
                output::tree::base(dir_entries.as_mut_slice(), ops.clone());
            } else if ops.contains(&Opti::List) {
                output::list::base(dir_entries.as_mut_slice(), ops.clone());
            } else {
                output::grid::base(vect_entry_name, &dir_entries);
            }
        }
    }
}
