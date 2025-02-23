use std::{fs, path::PathBuf};
use colored::{ColoredString, Colorize};
use zix_core::entry::{
    kind::EntryKind,
    Entry,
    create,
    create::Opti
};

enum TreeConnector   {
    Branch, // ├─
    LastBranch, // └──
    Vertical, // │
    Horizontal, // ─
    Empty // (space)
}

impl TreeConnector  {
    fn as_str(&self) -> ColoredString   {
        match self  {
            TreeConnector::Branch => "├──".bright_black(),
            TreeConnector::Vertical => "│".bright_black(),
            TreeConnector::Horizontal => "─".bright_black(),
            TreeConnector::LastBranch => "└──".bright_black(),
            TreeConnector::Empty => " ".bright_black()
        }
    }
}

fn recursive(na: &String, ki: EntryKind, et: i32, optis: Vec<Opti>, indent: &String) {
    if let EntryKind::Directory = ki {
        if let Ok(dir) = fs::read_dir(na) {
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
                                "{}{}",
                                indent,
                                if is_last { "    ".bright_black().to_string() } else { "│   ".bright_black().to_string() }
                            );

                            let full_path = entry.path().to_string_lossy().to_string();
                            if let Some(dir_entry) = create::dir(&entry, &optis) {
                                recursive(&full_path, dir_entry.entry_kind, et, optis.clone(), &new_indent);
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn base(items: &[Entry], ops: Vec<Opti>) {
    let mut w = 0;
    for (i, it) in items.iter().enumerate() {
        let is_last = i == items.len() - 1;

        println!(
            "{}{}",
            if is_last {
                TreeConnector::LastBranch.as_str()
            } else {
                TreeConnector::Branch.as_str()
            },
            it.name
        );

        let last = "    ".bright_black().to_string();
        let conti = "│   ".bright_black().to_string();
        recursive(
            &it.name,
            it.entry_kind.clone(),
            w,
            ops.clone(),
            if is_last { &last } else { &conti },
        );
        w += 1;
    }
}
