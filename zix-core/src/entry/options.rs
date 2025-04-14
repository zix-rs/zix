
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Opti   {
    All,
    List,
    Help,
    Version,
    Headers,
    Icons,
    Tree,
    Grid
}

impl Opti {
    pub fn as_flag(&self) -> &'static str {
        match self {
            Opti::All => "--all",
            Opti::List => "--list",
            Opti::Help => "--help",
            Opti::Version => "--version",
            Opti::Headers => "--headers",
            Opti::Icons => "--icons",
            Opti::Tree => "--tree",
            Opti::Grid => "--grid",
        }
    }

    pub fn as_short_flag(&self) -> Option<&'static str> {
        match self {
            Opti::All => Some("-a"),
            Opti::List => Some("-l"),
            Opti::Help => Some("-?"),
            Opti::Version => Some("-v"),
            Opti::Headers => Some("-h"),
            Opti::Icons => Some("-i"),
            Opti::Tree => Some("-t"),
            Opti::Grid => Some("-g"),
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Opti::All => "Show all items",
            Opti::List => "Display as a list",
            Opti::Help => "Show help message",
            Opti::Version => "Display version information",
            Opti::Headers => "Include headers in output",
            Opti::Icons => "Show icons",
            Opti::Tree => "Display as tree",
            Opti::Grid => "Display in grid format",
        }
    }
}
