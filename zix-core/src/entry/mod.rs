use std::path::PathBuf;
use kind::EntryKind;
use colored::Colorize;
use std::fs::Metadata;
use std::fs;
use unicode_width::UnicodeWidthStr;
pub mod create;
pub mod utils;
pub mod kind;
pub mod sets;

use crate::entry::utils::is_executable;
use crate::entry::sets::config_files;
use crate::entry::sets::is_file_in_set;

#[derive(Clone, Debug)]
pub struct Entry    {
    pub mode: String,
    pub last_modified: String,
    pub name: String,
    pub lenght: String,
    pub entry_kind: EntryKind,
    pub symlink: PathBuf,
    pub path: PathBuf,
    pub output_name: String,
    pub icon: String,
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
            icon: String::new()
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

    pub fn add_icon(&mut self) {
        let icon = self.entry_kind.icons(self.name.as_str()).to_string();
        self.icon = icon;
    }

    pub fn colored_name(&mut self) {
        let colored_name = match self.entry_kind {
            EntryKind::Hidden => self.name.red().bold(),
            EntryKind::Directory => self.name.green().bold(),
            EntryKind::Config => self.name.yellow().underline(),
            EntryKind::Symlink => self.name.bright_blue(),
            _ => self.name.normal(),
        };

        self.output_name = format!("{}{}", self.icon, colored_name);
    }

    pub fn entry_kind(&mut self, meta: Metadata, filename: &str) {
        let file_type = meta.file_type();

        self.entry_kind = if filename.starts_with('.') {
            EntryKind::Hidden
        } else if let Ok(target) = fs::read_link(self.path.clone()) {
            self.symlink = target;
            EntryKind::Symlink
        } else if file_type.is_file() {
            if filename.ends_with(".zip") || filename.ends_with(".tar") {
                EntryKind::Archive
            } else if is_executable(filename, &meta) {
                EntryKind::Executable
            } else if is_file_in_set(filename, &config_files()) {
                EntryKind::Config
            } else {
                EntryKind::File
            }
        } else if file_type.is_dir() {
            EntryKind::Directory
        } else {
            EntryKind::Other
        };
    }
}


pub use { create::dir, create::filter_dir };
