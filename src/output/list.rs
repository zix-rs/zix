use crate::{
    entry::{
        kind::EntryKind,
        Entry
    },
    output::utils::strip_ansi_codes
};
use colored::Colorize;
use crate::parser::Opti;

pub fn base(items: &mut [Entry], op: Vec<Opti>)   {
    let max_length = items.iter().map(|s| s.lenght.len()).max().unwrap_or(1) + 5;
    let ml = items.iter().map(|s| strip_ansi_codes(&s.lenght).len()).max().unwrap_or(1)+1;
    let empt = items.iter().all(|f| f.lenght == "-".bright_white().to_string());

    let mut output = String::new();

    if !items.is_empty()  {
        if op.contains(&Opti::Headers)  {
            #[cfg(windows)] {
                println!(
                    "{} {:<16} {:<w$} {}",
                    "Mode".bold().underline(), "Last Modified".bold().underline(), "Size".bold().underline(), "Name".bold().underline(), w = ml)
            }

            #[cfg(unix)]    {
                println!(
                    "{} {:<16} {:<w$} {}",
                    "Mode".bold().underline(), "Last Modified".bold().underline(), "Size".bold().underline(), "Name".bold().underline(), w = ml)
            }
        }
    }

    for entry in items.iter_mut()   {
            let v: Vec<&str> = entry.last_modified.split('\t').collect();
            if empt && entry.lenght == "-".bright_white().to_string() {
                entry.lenght = "   -".bright_white().to_string();
            }
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
                    output.push_str(
                        &format!(
                        "{:<6} {:<19} {:>width$} {}\n",
                        entry.mode, format!("{} {}", v[0].yellow(), v[1].green()), entry.lenght.bold(), entry.name, width = max_length
                        )
                    )
                }
            };
    }

    println!("{}", output)
}
