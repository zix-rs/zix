pub mod grid;
pub mod entry;
pub mod app;
pub mod ref_command;
pub mod parser;
pub mod window;
use app::App;
use grid::prgrid;
use parser::Opti;

fn main() {
    if let Some(app) = App::init() {
        let items = app.entries;
        let ops = app.options;
        let mut vect_entry_name: Vec<&str> = Vec::new();

        for na in &items   {
            vect_entry_name.push(&na.name);
        }

        if ops.contains(&Opti::List)    {
            prgrid::list( &items);
        } else {
            prgrid::base(&vect_entry_name);
        }
    }
}
