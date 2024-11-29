use crate::entry::{Entry, EntryKind};
use crate::window::get_terminal_size;
use unicode_width::UnicodeWidthStr;
use regex;
pub mod prgrid  {
    use super::*;
    pub enum Format {
        List,
        Base
    }

    fn strip_ansi_codes(input: &str) -> String {
        let ansi_regex = regex::Regex::new(r"\x1b\[[0-9;]*m").unwrap();
        ansi_regex.replace_all(input, "").to_string()
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

    pub fn base(ve: &Vec<&str>) {
        let stripped_names: Vec<String> = ve.iter().map(|name| strip_ansi_codes(name)).collect();
        let max_width = stripped_names
            .iter()
            .map(|name| UnicodeWidthStr::width(name.as_str()))
            .max()
            .unwrap_or(0)
            + 2;

        let (term_width, _) = get_terminal_size();
        let w = adjust_terminal_width(term_width);
        let columns = w / max_width.max(1);
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
        for row in 0..rows {
            for col in 0..columns {
                if let Some(name) = grid[col].get(row) {
                    print!("{:<width$}", name, width = column_widths[col] + 5);
                }
            }
            println!();
        }
    }
}
