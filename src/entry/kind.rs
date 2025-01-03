
#[derive(Clone, Debug)]
pub enum EntryKind {
    File,       // File
    Directory,  // Dir
    Symlink,    // Symbol Link
    Hidden,     // Hidden
    Executable, // .exe .app
    Archive,    // zip, tar, etc
    Config,     // config files
    Other,      // other
}

impl EntryKind {
    pub fn icons(self, filename: &str) -> &'static str  {
        match self {
            Self::File => {
                if filename.ends_with(".gitignore")  {
                    "\u{e702}"
                } else if filename.ends_with(".docx") {
                    "\u{f1392}"
                } else if filename.ends_with(".git") {
                    "\u{f1d3}"
                } else if filename.ends_with("README.md") {
                    "\u{eda4}"
                } else {
                    return "\u{f0219}"
                }
            },
            _ => ""
        }
    }
}
