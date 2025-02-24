use zix_core::entry::{
    Entry,
    kind::EntryKind
};
use colored::Colorize;
use zix_core::grid::out;

pub fn base(items: &[Entry]) {
    let mut vector: Vec<String> = vec![];

    for it in items {
        match it.entry_kind {
            EntryKind::Hidden =>   {
                let colored_name = it.name.red().bold().to_string();
                vector.push(colored_name);
            },
            EntryKind::Directory => {
                let colored_name = it.name.green().bold().to_string();
                vector.push(colored_name);
            },
            _ => {
                vector.push(it.name.to_string());
            }
        }
    }
    let output = out(vector);
    println!("{}", output);

}
