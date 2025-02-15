use std::path::PathBuf;
use kind::EntryKind;

pub mod create;
pub mod utils;
pub mod kind;


#[derive(Clone, Debug)]
pub struct Entry    {
    pub mode: String,
    pub last_modified: String,
    pub name: String,
    pub lenght: String,
    pub entry_kind: EntryKind,
    pub symlink: PathBuf,
    pub path: PathBuf
}

impl Entry  {
    pub fn new() -> Entry {
        Entry {
            mode: String::new(),
            last_modified: String::new(),
            name: String::new(),
            lenght: String::new(),
            entry_kind: EntryKind::Other,
            symlink: PathBuf::new(),
            path: PathBuf::new()
        }
    }
}

pub use { create::dir, create::filter_dir };
