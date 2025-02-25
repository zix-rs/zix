use std::{
    fs::{
        self,
        DirEntry
    },
    path::PathBuf
};
use chrono::{DateTime, Local};
use colored::Colorize;
use super::{kind::EntryKind, utils::{entry_mode, format_file_size, is_executable}, Entry};
use zix_utils::type_of;
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

pub fn filter_dir(path: &PathBuf) -> Option<Entry> {
    let meta = fs::metadata(path).ok()?;

    let mut entry_dir = Entry::new();
    entry_dir.name = path.to_string_lossy().to_string();
    entry_dir.lenght = format_file_size(meta.len());

    let permissions = meta.permissions();
    entry_dir.mode = entry_mode(meta.clone(), permissions);

    let modified_time = meta.modified().ok()?;
    let datetime: DateTime<Local> = modified_time.into();
    entry_dir.last_modified = datetime.format("%d %b %H:%M").to_string();
    entry_dir.colored_name();
    Some(entry_dir)
}


pub fn dir(dir_entry: &DirEntry, optis: &Vec<Opti>) -> Option<Entry> {
    let mut entry_dir = Entry::new();
    if let Some(filename) = dir_entry.file_name().to_str() {
        if filename.starts_with('.') && !optis.contains(&Opti::All) {
            return None;
        }
        entry_dir.path.push(dir_entry.path());

        if let Ok(metadata) = fs::symlink_metadata(dir_entry.path()) {
            if let Ok(target) = fs::read_link(&entry_dir.path) {
                entry_dir.entry_kind = EntryKind::Symlink;
                entry_dir.symlink = target;
            } else {
                entry_dir.entry_kind(metadata.clone(), filename);
            }

            entry_dir.lenght = if metadata.is_dir() {
                format!("{}", "-".bright_white())
            } else {
                format_file_size(metadata.len())
            };

            let permissions = metadata.permissions();
            entry_dir.mode = entry_mode(metadata.clone(), permissions);

            if let Ok(modified_time) = metadata.modified() {
                let datetime: DateTime<Local> = modified_time.into();
                entry_dir.last_modified = datetime.format("%d %b %H:%M").to_string();
            } else {
                println!("Couldn't retrieve the last modified time");
            }
        }

        entry_dir.name = filename.to_string();
        entry_dir.colored_name();
    }

    Some(entry_dir)
}
