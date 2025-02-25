use std::fmt::Display;
use zix_utils::{ansi, window};
use unicode_width::UnicodeWidthStr;

pub fn get_total_columns<T>(items: Vec<T>) -> usize
where
    T: Display + std::fmt::Debug + Clone,
{
    let (term_width, _) = window::get_terminal_size();
    let terminal_width = window::adjust_terminal_width(term_width);

    let mut current_width = 0;
    let mut columns_total: Vec<_> = vec![];
    let mut total_columns_pre = 0;
    let separator = UnicodeWidthStr::width("_");
    for item in items.iter()  {
        let item_str = item.to_string();
        let item_string = ansi::strip_ansi_codes(&item_str.as_str());
        let width = UnicodeWidthStr::width(item_string.as_str());

        if current_width + width + separator <= terminal_width.into()  {
            current_width += width + 10;
            total_columns_pre += 1;
        } else {
            columns_total.push(total_columns_pre);
            current_width = width + separator;
            total_columns_pre = 1;
        }
    }
    if total_columns_pre > 0 {
        columns_total.push(total_columns_pre);
    }

    let total_columns = if columns_total.is_empty() {
        1
    } else {
        *columns_total.iter().min().unwrap_or(&1).max(&1)
    };

    total_columns
}

/*
    header       header       header
    element      elementoss   sdafsdfssadf
    adsfasdfasd  fsadf        fdsafasdfds
    vec[column]  vec[column]  vec[column] <----- vec![column] <- column: ["header", "element", "adsfasdfasd"]
    column       column       column      <----- columns: 3
    vec[vec[]; columns]
*/
pub fn out<T: Display>(items: Vec<T>) -> String
where
    T: Display + std::fmt::Debug + Clone,
{
    let mut output = String::new();
    if items.len() == 0 {
        output.push_str("There are no items to display");
        return output
    }

    let total_columns = get_total_columns(items.clone());

    let mut grid: Vec<Vec<String>> = vec![Vec::new(); total_columns];

    let max_items = items.len();
    let mut i = 0;
    let mut j = 0;
    while i < max_items {
        grid[j].push(items[i].to_string());
        if j < total_columns - 1 {
            j += 1;
        } else {
            j = 0;
        }
        i += 1;
    }
    let rows = grid.iter().map(|f| f.len()).max().unwrap_or(0);

    for row in 0..rows {
        for col in 0..total_columns {
            let max_lenght_col = grid[col]
            .iter()
            .map(|f| ansi::strip_ansi_codes(f))
            .map(|name| UnicodeWidthStr::width(name.as_str()))
            .max()
            .unwrap_or(0)
            + 2;
            if let Some(item) = grid[col].get(row) {
                let stripped = ansi::strip_ansi_codes(item);
                let real_width = UnicodeWidthStr::width(stripped.as_str());
                let padding = max_lenght_col.saturating_sub(real_width);
                output.push_str(&format!("{}{}", item, " ".repeat(padding)));
            } else {
                output.push_str(&" ".repeat(max_lenght_col));
            }
        }
        output.push('\n')
    }

    output
}
