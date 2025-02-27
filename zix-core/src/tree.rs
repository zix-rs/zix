use colored::{ColoredString, Colorize};

pub enum TreeConnector {
    Branch, // ├─
    LastBranch, // └──
    Vertical, // │
    Empty // Empty
}

impl TreeConnector {
    pub fn as_str(&self) -> ColoredString {
        match self {
            TreeConnector::Branch => "├──".bright_black(),
            TreeConnector::Vertical => "│".bright_black(),
            TreeConnector::LastBranch => "└──".bright_black(),
            TreeConnector::Empty => " ".bright_black()
        }
    }
}
