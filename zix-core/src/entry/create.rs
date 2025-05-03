use std::{
    fs::{
        self,
        DirEntry
    },
    path::PathBuf
};
use chrono::{DateTime, Local};
use colored::Colorize;
use super::{
    kind::EntryKind,
    options::Opti,
    utils::{
        entry_mode,
        format_file_size
    },
    Entry
};

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

        if filename.len() > 26 {
            if !optis.contains(&Opti::List) {
                entry_dir.name = truncate_filename(&filename, 30);
            } else {
                entry_dir.name = filename.to_string();
            }
        } else {
            entry_dir.name = filename.to_string();
        }
        if optis.contains(&Opti::Icons) {
            entry_dir.add_icon();
        }
        entry_dir.colored_name();
    }

    Some(entry_dir)
}

fn truncate_filename(filename: &str, max_length: usize) -> String {
    if filename.len() <= max_length {
        return filename.to_string();
    }

    let prefix_len = max_length / 2;
    let suffix_len = max_length - prefix_len - 2;

    let prefix: String = filename.chars().take(prefix_len).collect();
    let suffix: String = filename.chars().rev().take(suffix_len).collect::<String>().chars().rev().collect();

    format!("{}..{}", prefix, suffix)
}
