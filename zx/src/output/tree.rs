use std::{fs, path::PathBuf};
use colored::{ColoredString, Colorize};
use zix_core::entry::{
    kind::EntryKind,
    Entry,
    create,
    create::Opti
};

enum TreeConnector {
    Branch, // ├─
    LastBranch, // └──
    Vertical, // │
    Empty // Empty
}

impl TreeConnector {
    fn as_str(&self) -> ColoredString {
        match self {
            TreeConnector::Branch => "├──".bright_black(),
            TreeConnector::Vertical => "│".bright_black(),
            TreeConnector::LastBranch => "└──".bright_black(),
            TreeConnector::Empty => " ".bright_black()
        }
    }
}

fn recursive(pa: &PathBuf, ki: EntryKind, optis: Vec<Opti>, indent: &String) {
    if let EntryKind::Directory = ki {
        if let Ok(dir) = fs::read_dir(pa) {
            let entries: Vec<_> = dir.collect();
            let total = entries.len();

            for (i, entry) in entries.into_iter().enumerate() {
                if let Ok(entry) = entry {
                    if let Ok(ft) = entry.file_type() {
                        let is_last = i == total - 1;
                        let name = entry.file_name().to_string_lossy().to_string();

                        println!(
                            "{}{} {}",
                            indent,
                            if is_last {
                                TreeConnector::LastBranch.as_str()
                            } else {
                                TreeConnector::Branch.as_str()
                            },
                            name
                        );

                        if ft.is_dir() {
                            let new_indent = format!(
                                "{}{}   ",
                                indent,
                                if is_last {
                                    TreeConnector::Empty.as_str()
                                } else {
                                    TreeConnector::Vertical.as_str()
                                }
                            );

                            if let Some(dir_entry) = create::dir(&entry, &optis) {
                                recursive(&dir_entry.path, dir_entry.entry_kind, optis.clone(), &new_indent);
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn base(items: &[Entry], ops: Vec<Opti>) {
    for (i, it) in items.iter().enumerate() {
        let is_last = i == items.len() - 1;

        match it.entry_kind {
            EntryKind::Directory => {
                println!(
                    "{} {}",
                    if is_last {
                        TreeConnector::LastBranch.as_str()
                    } else {
                        TreeConnector::Branch.as_str()
                    },
                    it.name.bright_green().bold()
                );
            },
            _ => {
                println!(
                    "{} {}",
                    if is_last {
                        TreeConnector::LastBranch.as_str()
                    } else {
                        TreeConnector::Branch.as_str()
                    },
                    it.name
                );
            }
        }

        let last = format!("{}   ", TreeConnector::Empty.as_str());
        let conti = format!("{}   ", TreeConnector::Vertical.as_str());
        recursive(
            &it.path,
            it.entry_kind.clone(),
            ops.clone(),
            if is_last { &last } else { &conti },
        );
    }
}
