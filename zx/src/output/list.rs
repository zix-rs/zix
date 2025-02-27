use zix_core::entry::{
    Entry,
    kind::EntryKind,
    create::Opti
};
use zix_core::grid::{out, get_total_columns, get_grid};
use colored::Colorize;
use zix_utils::ansi::strip_ansi_codes;

pub fn base(items: &mut [Entry], op: Vec<Opti>)   {
    let max_length = items.iter().map(|s| s.lenght.len()).max().unwrap_or(1) + 5;

    let ml = items.iter().map(|s| strip_ansi_codes(&s.lenght).len()).max().unwrap_or(1);
    let empt = items.iter().all(|f| f.lenght == "-".bright_white().to_string());

    let mut output = String::new();
    if !items.is_empty()  {
        if op.contains(&Opti::Headers) && !op.contains(&Opti::Grid)  {
            #[cfg(windows)]
            output.push_str(format!(
                "{} {:<13} {:<w$} {}\n",
                "Mode".bold().underline(),
                "Last Modified".bold().underline(),
                "Size".bold().underline(),
                "Name".bold().underline(),
                w = ml
            ));

            #[cfg(unix)]
            output.push_str(&format!(
                "{} {:<13} {:<w$} {}\n",
                "Permissions".bold().underline(),
                "Last Modified".bold().underline(),
                "Size".bold().underline(),
                "Name".bold().underline(),
                w = ml
            ));
        }
    }

    for entry in items.iter_mut()   {
            let v: Vec<&str> = entry.last_modified.split('\t').collect();
            let modified_display = if !v.is_empty() { v[0].yellow() } else { "N/A".yellow() };

            if empt && entry.lenght == "-".bright_white().to_string() {
                entry.lenght = "   -".bright_white().to_string();
            }

            #[cfg(windows)]
            output.push_str(
                &format!(
                  "{:<6} {:<11} {:>width$} {}\n",
                  entry.mode,
                  format!("{} ", modified_display),
                  entry.lenght.bold(),
                  entry.output_name,
                  width = max_length
                )
            );

            #[cfg(unix)]
            output.push_str(
                &format!(
                  "{}   {:<11} {:>width$} {}\n",
                  entry.mode,
                  modified_display,
                  entry.lenght.bold(),
                  entry.output_name,
                  width = max_length
                )
            );

    }

    let mut output_headers: Vec<String> = Vec::new();
    if op.contains(&Opti::Grid) {
        let binding = output.clone();
        let mut output_grid: Vec<String> = binding
            .split('\n')
            .map(|s| s.to_string())
            .collect();
        output_grid.retain(|line| !line.trim().is_empty());

        #[cfg(windows)]
        let header = format!(
            "{} {:<13} {:<w$} {}",
            "Mode".bold().underline(),
            "Last Modified".bold().underline(),
            "Size".bold().underline(),
            "Name".bold().underline(),
            w = ml
        );

        #[cfg(unix)]
        let header = format!(
            "{} {:<13} {:<w$} {}",
            "Permissions".bold().underline(),
            "Last Modified".bold().underline(),
            "Size".bold().underline(),
            "Name".bold().underline(),
            w = ml
        );
        let total_headers = get_total_columns(&output_grid);
        let grid = get_grid(output_grid.clone());
        let max_lengths: Vec<usize> = grid.iter()
        .map(|row| row.iter().map(|cell| strip_ansi_codes(cell.as_str()).len()).max().unwrap_or(0))
        .collect();

        for i in 0..total_headers {
            let padding = max_lengths[i].saturating_sub(strip_ansi_codes(header.as_str()).len());
            print!("{:>width$}{}  ", header.normal(), " ".repeat(padding), width = max_lengths[i]);
            if i == total_headers - 1   {
                print!("\n");
                break;
            }
        }

        output = out(output_grid.clone());
    }

    println!("{}", output)
}
