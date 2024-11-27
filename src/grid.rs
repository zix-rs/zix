use crate::entry::{Entry, EntryKind};
use crate::window::get_terminal_size;
use unicode_width::UnicodeWidthStr;

pub mod prgrid  {
    use super::*;
    pub enum Format {
        List,
        Base
    }

    fn adjust_terminal_width(term_width: u16) -> usize  {
        if term_width >= 80 {
            <u16 as Into<usize>>::into(term_width) - 20
        } else {
            <u16 as Into<usize>>::into(term_width)
        }

    }

    pub fn list(items: &[Entry])   {
        for entry in items.iter()   {
                println!(
                    "{:<6} \t {:<19} {:>8} {}",
                    entry.mode, entry.last_modified, entry.lenght, entry.name
                );
        }
    }

    pub fn base(ve: &Vec<&str>, entries: &Vec<Entry>)  {
        let max_width = ve
        .iter()
        .map(|name| UnicodeWidthStr::width(*name))
        .max()
        .unwrap_or(0)
        + 2;

        let (term_width, _) = get_terminal_size();

        let w =  adjust_terminal_width(term_width);
        let columns = w / max_width;

        for (i, name) in ve.iter().enumerate() {
            match entries[i].entry_kind {
                EntryKind::Archive  => { print!("{:<width$}", name, width = max_width) },
                EntryKind::Directory => { print!("{:<width$}", name, width = max_width) },
                EntryKind::Config  => { print!("{:<width$}", name, width = max_width) },
                EntryKind::Executable  => { print!("{:<width$}", name, width = max_width) },
                EntryKind::File  => { print!("{:<width$}", name, width = max_width) },
                EntryKind::Hidden  => { print!("{:<width$}", name, width = max_width) },
                EntryKind::Symlink  => { print!("{:<width$}", name, width = max_width) },
                _ => print!("{:<width$}", name, width = max_width)
            }

            if (i + 1) % columns == 0 {
                println!();
            }
        }
        println!();
    }
}
