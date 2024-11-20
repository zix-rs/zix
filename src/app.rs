use std::fs::{self, DirEntry};
use std::os::windows::fs::MetadataExt;
use std::path::PathBuf;
use chrono::{DateTime, Local};
use crate::entry::{create_entry, create_entry_for_dir, is_executable, Entry, EntryKind};
use crate::parser::{parse, Opti};
use crate::ref_command::*;
use glob::glob;

#[derive(Clone, Debug)]
pub struct App {
    pub entries: Vec<Entry>,
    pub dirs: Vec<String>,
    pub name: &'static str,
    pub version: &'static str,
    pub options: Vec<Opti>

}

impl App    {
    pub fn init() -> Option<App>    {
        let mut app = App {
            entries: Vec::new(),
            dirs: Vec::new(),
            name: &NAME,
            version: &VERSION,
            options: Vec::new()
        };

        let (options, values) = parse();
        let mut entries: Vec<Entry> = Vec::new();
        let mut dirs: Vec<String> = Vec::new();
        for op in options   {
            match op.as_str()    {
                "--help" | "-h" => { help(); return None},
                "--version" | "-v" => { version(); return None},
                "--all" | "-a" => app.options.push(Opti::All),
                "--list" | "-l" => app.options.push(Opti::List),
                _ => todo!()
            }
        }

        for val in values.iter()   {
            if val.contains('*')    {

                if let Ok(paths) = glob(&val){
                    entries.extend(
                        paths
                            .filter_map(Result::ok)
                            .filter_map(|path| create_entry(&path))
                    );
                }

            } else {
                if let Ok(dir) = fs::read_dir::<&String>(&val)   {
                    entries.extend(
                        dir
                                .filter_map(Result::ok)
                                .filter_map(|path| create_entry_for_dir(&path, &app.options))
                    );
               } else {
                   continue;
               };
            }
        }

        app.entries = entries;
        app.dirs = dirs;
        Some(app)
    }
}
