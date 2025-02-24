use terminal_size::{Width, Height, terminal_size};

pub fn get_terminal_size() -> (u16, u16) {
    if let Some((Width(w), Height(h))) = terminal_size() {
        (w, h)
    } else {
        (80, 24)
    }
}

pub fn adjust_terminal_width(term_width: u16) -> usize  {
    let adjusted_width = if term_width >= 150 {
        term_width.saturating_sub(20)
    } else {
        term_width
    };
    adjusted_width.max(1).into()
}
