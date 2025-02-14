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

pub fn filter_entries<O: FilterOptions>(
    entries: &[Entry],
    options: &O,
) -> Vec<Entry> {
    entries
        .iter()
        .filter(|entry| {
            // Incluir archivos ocultos solo si la opción `All` está activa
            !entry.name.starts_with('.') || options.should_include_hidden()
        })
        .cloned()
        .collect()
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
    entry_dir.last_modified = datetime.format("%d/%m/%Y\t%H:%M").to_string();

    Some(entry_dir)
}

pub trait FilterOptions {
    fn should_include_hidden(&self) -> bool;
    fn should_show_icons(&self) -> bool;
    fn should_use_tree_view(&self) -> bool;
    fn should_use_list_view(&self) -> bool;
    fn should_use_grid_view(&self) -> bool;
}


pub fn dir<O: FilterOptions>(
    dir_entry: &DirEntry,
    options: &O,
) -> Option<Entry> {
    let mut entry_dir = Entry::new();
    if let Some(filename) = dir_entry.file_name().to_str() {
        // Usar el trait para decidir si incluir archivos ocultos
        if filename.starts_with('.') && !options.should_include_hidden() {
            return None;
        }

        entry_dir.path.push(dir_entry.path());
        if let Ok(metadata) = fs::metadata(dir_entry.path()) {
            entry_dir.lenght = if metadata.is_dir() {
                format!("{}", "-".bright_white())
            } else {
                format_file_size(metadata.len())
            };
            let permissions = metadata.permissions();

            entry_dir.mode = entry_mode(metadata.clone(), permissions);
            if let Ok(modified_time) = metadata.modified() {
                let datetime: DateTime<Local> = modified_time.into();
                entry_dir.last_modified = datetime.format("%d/%m/%Y\t%H:%M").to_string()
            } else {
                println!("Couldn't retrieve the last modified time")
            };

            let ft = metadata.file_type();
            let pat = filename.to_string();
            if ft.is_file() {
                if pat.ends_with(".zip") || pat.ends_with(".tar") {
                    entry_dir.entry_kind = EntryKind::Archive
                } else if pat.ends_with(".conf") || pat.ends_with(".config") {
                    entry_dir.entry_kind = EntryKind::Config
                } else if is_executable(filename, &metadata) {
                    entry_dir.entry_kind = EntryKind::Executable
                } else {
                    entry_dir.entry_kind = EntryKind::File
                }

                match fs::read_link(dir_entry.path()) {
                    Ok(target) => {
                        entry_dir.entry_kind = EntryKind::Symlink;
                        entry_dir.symlink = target
                    },
                    Err(_) => {},
                }
            } else if ft.is_dir() {
                entry_dir.entry_kind = EntryKind::Directory
            } else {
                entry_dir.entry_kind = EntryKind::Other
            }
        }

        if filename.starts_with('.') {
            entry_dir.entry_kind = EntryKind::Hidden
        }

        entry_dir.name = filename.to_string();
    }

    Some(entry_dir)
}
