use std::path::PathBuf;
use kind::EntryKind;
use colored::Colorize;
use std::fs::Metadata;
use std::fs;

pub mod create;
pub mod utils;
pub mod kind;
pub mod sets;

use crate::entry::utils::is_executable;
use crate::entry::sets::config_files;
use crate::entry::sets::is_config_file;

#[derive(Clone, Debug)]
pub struct Entry    {
    pub mode: String,
    pub last_modified: String,
    pub name: String,
    pub lenght: String,
    pub entry_kind: EntryKind,
    pub symlink: PathBuf,
    pub path: PathBuf,
    pub output_name: String
}

impl Default for Entry {
    fn default() -> Self {
        Self {
            mode: String::new(),
            last_modified: String::new(),
            name: String::new(),
            lenght: String::new(),
            entry_kind: EntryKind::Other,
            symlink: PathBuf::new(),
            path: PathBuf::new(),
            output_name: String::new(),
        }
    }
}

impl Entry  {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn shorten_name(&mut self) {
        let mut shorten_name = String::new();
        if self.name.chars().count() > 25 {
            let start: String = self.name.chars().take(5).collect();
            let end: String = self.name.chars().rev().take(7).collect::<String>().chars().rev().collect();
            shorten_name = format!("{}...{}", start, end)
        } else {
            shorten_name = self.name.to_string();
        }
        self.name = shorten_name
    }

    pub fn colored_name(&mut self)  {
        self.output_name = match self.entry_kind {
            EntryKind::Hidden =>  self.name.red().bold().to_string(),
            EntryKind::Directory => self.name.green().bold().to_string(),
            EntryKind::Config => self.name.yellow().underline().to_string(),
            EntryKind::Symlink => self.name.bright_blue().to_string(),
            _ => self.name.normal().to_string()
        };
    }

    pub fn entry_kind(&mut self, meta: Metadata, filename: &str) {
        let pat = filename.to_string();
        let file_type = meta.file_type();
        if filename.starts_with('.') {
            self.entry_kind = EntryKind::Hidden;
            return;
        }

        if let Ok(target) = fs::read_link(self.path.clone()) {
            self.entry_kind = EntryKind::Symlink;
            self.symlink = target;
            return;
        }

        if file_type.is_file() {
            if pat.ends_with(".zip") || pat.ends_with(".tar") {
                self.entry_kind = EntryKind::Archive;
            } else if is_executable(filename, &meta) {
                self.entry_kind = EntryKind::Executable;
            } else {
                self.entry_kind = EntryKind::File;
            }

            if is_config_file(filename, &config_files()) {
                self.entry_kind = EntryKind::Config
            }

        } else if file_type.is_dir() {
            self.entry_kind = EntryKind::Directory;
        } else {
            self.entry_kind = EntryKind::Other;
        }
    }
}


pub use { create::dir, create::filter_dir };
