pub mod app;
pub mod output;
pub mod ref_command;
pub mod window;

use app::App;
use zix_core::entry::create::Opti;

fn main() {
    if let Some(app) = App::init() {
        let items = app.entries;
        let ops = app.options;
        let mut vect_entry_name: Vec<String> = Vec::new();

        for na in &items   {
            vect_entry_name.push(na.name.clone());
        }

        if ops.contains(&Opti::Tree)  {
            output::tree::base(&items, ops);
        } else if ops.contains(&Opti::List) {
            output::list::base(&items, ops);
        } else {
            output::grid::base(vect_entry_name, &items);
        }
    }
}
