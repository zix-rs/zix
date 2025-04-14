use std::collections::HashMap;

#[derive(Clone, Debug , PartialEq)]
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
    pub fn icons(&self, filename: &str) -> &'static str {
        match self {
            Self::File => Self::file_icon(filename),
            Self::Directory => "\u{e5ff}",
            Self::Symlink => "\u{f838}",
            Self::Hidden => "\u{f023}",
            Self::Executable => "\u{f489}",
            Self::Archive => "\u{f187}",
            Self::Config => "\u{e5fc}",
            Self::Other => "\u{f128}",
        }
    }

    fn file_icon(filename: &str) -> &'static str {
        let icon_map: HashMap<&str, &str> = HashMap::from([
            (".gitignore", "\u{e702}"),
            (".docx", "\u{f1392}"),
            (".git", "\u{f1d3}"),
            ("README.md", "\u{eda4}"),
        ]);

        for (extension, icon) in icon_map {
            if filename.ends_with(extension) {
                return icon;
            }
        }

        "\u{f0219}" // Default
    }
}
