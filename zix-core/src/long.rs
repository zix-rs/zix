use colored::Colorize;
use zix_utils::{ansi, window};

use crate::entry::Entry;

/**
 * Mode Last Modified Size Name
 * -arw 13 Apr 21:49  37 B .gitignore
 * /-------- Meta --------|---Name---|
 * -arw 13 Apr 21:49  37 B Weinberg S. Gravitation and cosmology.. principles
 *                         and applications of GR (Wiley, 1972)(ISBN)(400dpi)
 *                         (T)(685s)_PGr_.djvu
 * Considerar los caracteres ANSI
 * |-------------------- console ---------------------|
 * |----------- meta ---------|---Name---|
 * if is long
 * |-------------------- console ---------------------|
 * |----------- meta ---------|------------Name-------|
 * |--------------------------|-----------Name--------|
**/
pub fn long(entry: &mut Entry, empt: bool, max_length: usize) -> String {
    let v: Vec<&str> = entry.last_modified.split('\t').collect();
    let modified_display = if !v.is_empty() { v[0].yellow() } else { "N/A".yellow() };

    if empt && entry.lenght == "-".bright_white().to_string() {
        entry.lenght = "   -".bright_white().to_string();
    }
    let (w, _h) = window::get_terminal_size();

    
    let meta = format!(
        "{:<6} {:<11} {:>width$}",
        entry.mode,
        modified_display,
        entry.lenght.bold(),
        width = max_length
    );

    let name = entry.output_name.clone();
    let meta_long = ansi::strip_ansi_codes(&meta).len();
    let name_long = ansi::strip_ansi_codes(&name).len();

    let mut out_name = String::new();

    if meta_long + name_long > w as usize {
        out_name.push_str( meta.as_str() );
        out_name.push(' ');

        for c in ansi::strip_ansi_codes(&name).chars() {
            if c == '\n' {
                out_name.push_str(&format!(" {} ", name));
            } else {
                out_name.push(c);
            }
        }
    } else {
        out_name.push_str(&format!("{} {}", meta, name));
    }

    out_name.push('\n');
    out_name
}
