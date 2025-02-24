use zix_core::{
    entry::{
        create::Opti,
        kind::EntryKind,
        Entry
    },
    grid::{
        get_total_columns,
        out
    }
};
use std::fmt::Write as FmtWrite;
use colored::Colorize;
use zix_utils::{ansi::strip_ansi_codes, window};
use unicode_width::UnicodeWidthStr;

pub fn base(items: &mut [Entry], op: Vec<Opti>) {
    let mut output = String::new();

    let max_length = items.iter().map(|s| s.lenght.len()).max().unwrap_or(1) + 5;
    let ml = items.iter().map(|s| strip_ansi_codes(&s.lenght).len()).max().unwrap_or(1) + 1;
    let empt = items.iter().all(|f| f.lenght == "|".bright_white().to_string());

    // <_ grid _>
    let mut output_grid: Vec<String> = vec![];

    if !items.is_empty() && op.contains(&Opti::Headers) && !op.contains(&Opti::Grid) {
        write!(
            &mut output,
            "{} {:<13} {:<w$} {}\n",
            "Mode".bold().underline(),
            "Last Modified".bold().underline(),
            "Size".bold().underline(),
            "Name".bold().underline(),
            w = ml
        ).unwrap();
    }

    for entry in items.iter_mut() {
        let v: Vec<&str> = entry.last_modified.split('\t').collect();

        if empt && entry.lenght == "-".bright_white().to_string() {
            entry.lenght = "   -".bright_white().to_string();
        }
        let entry_name = match entry.entry_kind {
            EntryKind::Directory => entry.name.bright_green().bold(),
            EntryKind::Hidden => entry.name.bright_red().bold(),
            _ => entry.name.white(),
        };

        let formatted = match entry.entry_kind {
            EntryKind::Symlink => {
                let symln = entry.symlink.to_string_lossy().replace("\\", "/");
                format!(
                    "{:<6} {:<19} {:>width$} {}",
                    entry.mode,
                    format!("{} ", v[0].yellow()),
                    entry.lenght.bold(),
                    entry_name.bright_blue().bold(),
                    width = max_length
                )
            },
            _ => format!(
                "{:<6} {:<19} {:>width$} {}",
                entry.mode,
                format!("{} ", v[0].yellow()),
                entry.lenght.bold(),
                entry_name,
                width = max_length
            ),
        };

        output_grid.push(formatted);
    }

    let max_item_width = output_grid
        .iter()
        .map(|item| UnicodeWidthStr::width(strip_ansi_codes(item).as_str()))
        .max()
        .unwrap_or(0);

    let total_columns = get_total_columns(max_item_width);

    if op.contains(&Opti::Grid) {
        if op.contains(&Opti::Headers) {
            let header = format!(
                "{} {:<13} {:<w$} {}",
                "Mode".bold().underline(),
                "Last Modified".bold().underline(),
                "Size".bold().underline(),
                "Name".bold().underline(),
                w = ml
            );
            for _ in 0..total_columns {
                output_grid.insert(0, header.clone());
            }
        }

        let grid_output = out(output_grid);
        write!(&mut output, "{}\n", grid_output).unwrap();
    } else {
        for line in output_grid {
            write!(&mut output, "{}\n", line).unwrap();
        }
    }

    print!("{}", output)
}
