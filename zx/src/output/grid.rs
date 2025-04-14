use zix_core::entry::{
    Entry,
    kind::EntryKind
};
use colored::Colorize;
use zix_core::grid::out;

pub fn base(items: &[Entry]) {
    let mut vector: Vec<String> = vec![];

    for it in items {
        vector.push(it.output_name.clone());
    }

    let output = out(vector);
    println!("{}", output);
}
