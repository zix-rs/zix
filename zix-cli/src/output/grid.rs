use zix_core::entry::{
    Entry,
    kind::EntryKind
};
use colored::Colorize;
use unicode_width::UnicodeWidthStr;
use super::utils::*;
use crate::window::get_terminal_size;


pub enum Format {
    List,
    Base
}


pub fn base(ve: Vec<String>, items: &[Entry]) {
    let stripped_names: Vec<String> = ve.iter().map(|name| strip_ansi_codes(name)).collect();
    let max_width = stripped_names
        .iter()
        .map(|name| UnicodeWidthStr::width(name.as_str()))
        .max()
        .unwrap_or(0)
        + 2;

    let (term_width, _) = get_terminal_size();
    let w = adjust_terminal_width(term_width);
    let columns = (w / max_width.max(1)).max(1);
    let rows = (ve.len() + columns - 1) / columns;

    let mut column_widths = vec![0; columns];
    let mut grid: Vec<Vec<String>> = vec![vec![]; columns];

    for (i, name) in stripped_names.iter().enumerate() {
        let col = i / rows;
        if grid[col].len() < rows {
            grid[col].push(name.clone());
        }
        column_widths[col] = column_widths[col].max(UnicodeWidthStr::width(name.as_str()));

    }
    let mut i = 0;
    for row in 0..rows {
        for col in 0..columns {
            if let Some(_) = grid[col].get(row) {
                match items[i].entry_kind {
                    EntryKind::Archive =>   {
                        print!(" {:<width$} ", items[i].name, width = column_widths[col] + 5);
                    },
                    EntryKind::Directory => {
                        print!(" {:<width$} ", items[i].name.clone().green(), width = column_widths[col] + 5);
                    },
                    _ => {
                        print!(" {:<width$} ", items[i].name, width = column_widths[col] + 5);
                    }
                }
                i += 1;

            }
        }
        println!();
    }
}
