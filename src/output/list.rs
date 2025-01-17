use crate::entry::{
    create::Entry,
    kind::EntryKind
};
use colored::Colorize;
use crate::parser::Opti;

pub fn base(items: &[Entry], op: Vec<Opti>)   {
    let max_length = items.iter().map(|s| s.lenght.len()).max().unwrap_or(1) + 5;
    let mut output = String::new();

    if !items.is_empty()  {
        if op.contains(&Opti::Headers)  {
            #[cfg(windows)] {
                println!(
                    "{:<6} {:<15} {:<8} {}",
                    "Mode".bold(), "Last Modified".bold(), "Size".bold(), "Name".bold())
            }

            #[cfg(unix)]    {
                println!(
                    "{:<11} {:<15} {:<8} {}",
                    "Mode".bold(), "Last Modified".bold(), "Size".bold(), "Name".bold())
            }
        }
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
