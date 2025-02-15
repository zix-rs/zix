use terminal_size::{Width, Height, terminal_size};

pub fn get_terminal_size() -> (u16, u16) {
    if let Some((Width(w), Height(h))) = terminal_size() {
        (w, h)
    } else {
        (80, 24)
    }
}
