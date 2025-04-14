use std::fmt::Display;
use zix_utils::{ansi, window};
use unicode_width::UnicodeWidthStr;

pub fn get_total_columns<T>(items: &[T]) -> usize
where
    T: Display + std::fmt::Debug + Clone,
{
    let (term_width, _) = window::get_terminal_size();
    let terminal_width = window::adjust_terminal_width(term_width);

    let max_item_width = items
        .iter()
        .map(|item| {
            let stripped = ansi::strip_ansi_codes(&item.to_string());
            UnicodeWidthStr::width(stripped.as_str()) + 2
        })
        .max()
        .unwrap_or(0);

    if max_item_width == 0 {
        return 1;
    }

    let total_columns = (terminal_width / max_item_width).max(1);
    total_columns.min(items.len())
}

/*
    header       header       header
    element      elementoss   sdafsdfssadf
    adsfasdfasd  fsadf        fdsafasdfds
    vec[column]  vec[column]  vec[column] <----- vec![column] <- column: ["header", "element", "adsfasdfasd"]
    column       column       column      <----- columns: 3
    vec[vec[]; columns]
*/
pub fn get_grid<T>(items: Vec<T>) -> Vec<Vec<String>>
where
    T: Display + std::fmt::Debug + Clone,
{
    let total_columns = get_total_columns(&items);
    let total_items = items.len();

    let total_rows = (total_items + total_columns - 1) / total_columns;

    let mut grid: Vec<Vec<String>> = vec![Vec::with_capacity(total_rows); total_columns];

    for (idx, item) in items.into_iter().enumerate() {
        let col = idx % total_columns;
        grid[col].push(item.to_string());
    }

    grid
}
pub fn out<T>(items: Vec<T>) -> String
where
    T: Display + std::fmt::Debug + Clone,
{
    let mut output = String::new();
    if items.is_empty() {
        output.push_str("There are no items to display");
        return output;
    }

    let grid = get_grid(items);
    let total_columns = grid.len();
    if total_columns == 0 {
        return output;
    }

    let col_widths: Vec<usize> = grid
        .iter()
        .map(|column| {
            column
                .iter()
                .map(|item| {
                    let stripped = ansi::strip_ansi_codes(item);
                    UnicodeWidthStr::width(stripped.as_str())
                })
                .max()
                .unwrap_or(0) + 2 
        })
        .collect();

    let total_rows = grid[0].len();

    for row in 0..total_rows {
        for col in 0..total_columns {
            if let Some(item) = grid[col].get(row) {
                let stripped = ansi::strip_ansi_codes(item);
                let width = UnicodeWidthStr::width(stripped.as_str());
                let padding = col_widths[col].saturating_sub(width);
                output.push_str(item);
                output.push_str(&" ".repeat(padding));
            } else {
                output.push_str(&" ".repeat(col_widths[col]));
            }
        }
        output.push('\n');
    }

    output
}
