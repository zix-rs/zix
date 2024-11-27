use std::{fs::{self, DirEntry}, os::windows::fs::MetadataExt, path::PathBuf};

use chrono::{DateTime, Local};

use crate::parser::Opti;

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

#[derive(Clone, Debug)]
pub struct Entry    {
    pub mode: String,
    pub last_modified: String,
    pub name: String,
    pub lenght: u64,
    pub father: String,
    pub entry_kind: EntryKind
}

impl Entry  {
    pub fn new() -> Entry {
        Entry {
            mode: String::new(),
            last_modified: String::new(),
            name: String::new(),
            lenght: 0,
            father: String::new(),
            entry_kind: EntryKind::Other
        }
    }
}

pub fn is_executable(filename: &str, _metadata: &fs::Metadata) -> bool {
    if filename.ends_with(".exe") || filename.ends_with(".bat") || filename.ends_with(".cmd") {
        return true;
    }
    false
}

pub fn create_entry(path: &PathBuf) -> Option<Entry> {
    let meta = fs::metadata(path).ok()?;

    let mut entry_dir = Entry::new();
    entry_dir.name = path.to_string_lossy().to_string();
    entry_dir.lenght = meta.len();

    let permissions = meta.permissions();
    entry_dir.mode = format!(
        "{}{}{}{}",
        if meta.is_dir() { "d" } else {"-"},
        if meta.is_file() { "a" } else {"-"},
        if permissions.readonly() { "r" } else { "-" },
        "-"
    );

    let modified_time = meta.modified().ok()?;
    let datetime: DateTime<Local> = modified_time.into();
    entry_dir.last_modified = datetime.format("%d/%m/%Y\t%H:%M").to_string();

    Some(entry_dir)
}


pub fn create_entry_for_dir(dir_entry: &DirEntry, optis: &Vec<Opti>) -> Option<Entry> {
    let mut entry_dir = Entry::new();
    if let Some(filename) = dir_entry.file_name().to_str()    {
        if filename.starts_with('.') && !optis.contains(&Opti::All) {
            return None;
        }

        if let Ok(metadata) = fs::metadata(dir_entry.path())    {
            entry_dir.lenght = metadata.file_size();
            let permissions = metadata.permissions();
            entry_dir.mode = format!(
                "{}{}{}{}",
                if metadata.is_dir() { "d" } else {"-"},
                if metadata.is_file() { "a" } else {"-"},
                if permissions.readonly() { "r" } else { "-" },
                "-"
            );
            if let Ok(modified_time) = metadata.modified()  {
                let datetime: DateTime<Local> = modified_time.into();
                entry_dir.last_modified = datetime.format("%d/%m/%Y\t%H:%M").to_string()
            } else {
                println!("Couldn't retrieve the last modified time")
            };

            let ft = metadata.file_type();
            let pat = filename.to_string();
            if ft.is_file() {
                if pat.ends_with(".zip") || pat.ends_with(".tar")   {
                    entry_dir.entry_kind = EntryKind::Archive
                } else if pat.ends_with(".conf") || pat.ends_with(".config")    {
                    entry_dir.entry_kind = EntryKind::Config
                } else if is_executable(filename, &metadata)    {
                    entry_dir.entry_kind = EntryKind::Executable
                } else {
                    entry_dir.entry_kind = EntryKind::File
                }
            } else if ft.is_dir()   {
                entry_dir.entry_kind = EntryKind::Directory
            } else if ft.is_symlink()   {
                entry_dir.entry_kind = EntryKind::Symlink
            } else  {
                entry_dir.entry_kind = EntryKind::Other
            }
        }

        entry_dir.name = filename.to_string();
    }

    Some(entry_dir)
}
