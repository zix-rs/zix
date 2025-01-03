use crate::entry::{
    create::Entry,
    kind::EntryKind
};
use crate::window::get_terminal_size;
use unicode_width::UnicodeWidthStr;
use regex;
use colored::Colorize;

pub mod prgrid  {
    use crate::parser::Opti;

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
        let adjusted_width = if term_width >= 80 {
            term_width.saturating_sub(20)
        } else {
            term_width
        };
        adjusted_width.max(1).into()
    }

    pub fn list(items: &[Entry], op: Vec<Opti>)   {
        let max_length = items.iter().map(|s| s.lenght.len()).max().unwrap_or(1) + 5;

        let mut output = String::new();

        if op.contains(&Opti::Headers)  {
            println!(
                "{:<6} {:<15} {:<7} {}",
                "Mode".bold(), "Last Modified".bold(), "Size".bold(), "Name".bold())
        }

        for entry in items.iter()   {
                let v: Vec<&str> = entry.last_modified.split('\t').collect();
                match entry.entry_kind  {
                    EntryKind::Hidden => {
                        output.push_str(
                    &format!(
                            "{:<6} {:<19} {:>width$} {}\n",
                            entry.mode, format!("{} {}", v[0].yellow(), v[1].green()), entry.lenght.bold(), entry.name.bright_red().bold(), width = max_length)
                        );
                    },
                    EntryKind::Directory => {
                        output.push_str(&format!(
                            "{:<6} {:<19} {:>width$} {}\n",
                            entry.mode, format!("{} {}", v[0].yellow(), v[1].green()), entry.lenght.bold(), entry.name.bright_green().bold(), width = max_length)
                        )
                    },
                    EntryKind::Symlink => {
                        let symln = entry.symlink.to_string_lossy().replace("\\", "/");
                        output.push_str(
                        &format!(
                            "{:<6} {:<19} {:>width$} {} → {}\n",
                            entry.mode, format!("{} {}", v[0].yellow(), v[1].green()), entry.lenght.bold(), entry.name, symln, width = max_length)
                            )
                        },
                    _ => {
                        output.push_str(&format!(
                            "{:<6} {:<19} {:>width$} {}\n",
                            entry.mode, format!("{} {}", v[0].yellow(), v[1].green()), entry.lenght.bold(), entry.name, width = max_length)
                        )
                    }
                };
        }

        println!("{}", output)
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
}
