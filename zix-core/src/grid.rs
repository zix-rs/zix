use std::fmt::Display;
use zix_utils::{ansi, window};
use unicode_width::UnicodeWidthStr;

pub fn get_total_columns(max_item_width: usize) -> i16 {
    let (term_width, _) = window::get_terminal_size();
    let terminal_width = window::adjust_terminal_width(term_width);
    let separator = UnicodeWidthStr::width("_");

    let max_columns = (terminal_width as usize / (max_item_width + separator)).max(1);
    max_columns as i16
}

pub fn out<T>(items: Vec<T>) -> String
where
    T: Display + std::fmt::Debug + Clone,
{
    let stripped_items: Vec<(String, usize)> = items
        .into_iter()
        .map(|item| {
            let item_str = item.to_string();
            let stripped = ansi::strip_ansi_codes(&item_str);
            let width = UnicodeWidthStr::width(stripped.as_str());
            (item_str, width)
        })
        .collect();

    let max_item_width = stripped_items.iter().map(|(_, width)| *width).max().unwrap_or(0);

    let total_columns = get_total_columns(max_item_width) as usize;
    let total_columns = total_columns.max(1);

    let mut output = String::new();
    let mut grid: Vec<Vec<String>> = vec![Vec::new(); total_columns];

    for (i, (item_str, _)) in stripped_items.iter().enumerate() {
        let col = i % total_columns;
        grid[col].push(item_str.clone());
    }

    let mut max_col_widths: Vec<(usize, usize)> = grid
        .iter()
        .enumerate()
        .map(|(col_index, column)| {
            let max_width = column
                .iter()
                .map(|item| UnicodeWidthStr::width(ansi::strip_ansi_codes(item).as_str()))
                .max()
                .unwrap_or(0)
                + 2;
            (col_index, max_width)
        })
        .collect();

    max_col_widths.sort_by(|a, b| b.1.cmp(&a.1));

    let mut sorted_grid: Vec<Vec<String>> = Vec::with_capacity(total_columns);
    for (col_index, _) in max_col_widths.iter() {
        sorted_grid.push(grid[*col_index].clone());
    }

    let rows = sorted_grid.iter().map(|f| f.len()).max().unwrap_or(0);
    for row in 0..rows {
        for col in 0..total_columns {
            if let Some(item) = sorted_grid[col].get(row) {
                let stripped = ansi::strip_ansi_codes(item);
                let real_width = UnicodeWidthStr::width(stripped.as_str());
                let padding = if col < total_columns - 1 {
                    max_col_widths[col].1.saturating_sub(real_width)
                } else {
                    0
                };
                let truncated_item = if real_width > max_col_widths[col].1 {
                    let mut truncated = item.chars().take(max_col_widths[col].1 - 3).collect::<String>();
                    truncated.push_str("...");
                    truncated
                } else {
                    item.clone()
                };
                output.push_str(&format!("{}{}", truncated_item, " ".repeat(padding)));
            } else {
                output.push_str(&" ".repeat(max_col_widths[col].1));
            }
        }
        output.push('\n');
    }

    output
}
