use zix_core::entry::Entry;
use zix_core::grid;

pub fn base(items: &[Entry]) {
    let mut vector: Vec<String> = vec![];

    for it in items {
        vector.push(it.output_name.clone());
    }

    let grid = grid::out(vector.clone());
    println!("{}", grid)
}
