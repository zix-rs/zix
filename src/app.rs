use std::fs;
use std::os::windows::fs::MetadataExt;
use chrono::{DateTime, Local};
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
#[derive(Clone, Debug)]
pub struct Entry    {
    pub mode: String,
    pub last_modified: String,
    pub name: String,
    pub lenght: u64,
    pub father: String,
}
impl Entry  {
    pub fn new() -> Entry {
        Entry {
            mode: String::new(),
            last_modified: String::new(),
            name: String::new(),
            lenght: 0,
            father: String::new(),
        }
    }
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


        for val in values   {
            if val.contains('*')    {

                if let Ok(paths) = glob(&val){
                    println!("{:#?}",paths);
                    for path in paths   {
                        if let Ok(path) = path  {
                            if let Ok(meta) = fs::metadata(&path)   {
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
                                if let Ok(modified_time) = meta.modified()  {
                                    let datetime: DateTime<Local> = modified_time.into();
                                    entry_dir.last_modified = datetime.format("%d/%m/%Y\t%H:%M").to_string()
                                } else {
                                    println!("Couldn't retrieve the last modified time")
                                }
                                entries.push(entry_dir);
                            }
                        }
                    }
                }

            } else {
                if let Ok(dir) = fs::read_dir::<&String>(&val)   {
                    dirs.push(val.clone());
                    for entry in dir    {
                        let mut entry_dir = Entry::new();
                        if let Ok(entry) = entry    {
                            entry_dir.father = val.clone();
                            if let Some(filename) = entry.file_name().to_str()  {
                                if filename.starts_with('.') && !app.options.contains(&Opti::All) {
                                    continue;
                                }
                                if let Ok(metadata) = fs::metadata(entry.path())    {
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
                                    }
                                }
                                entry_dir.name = filename.to_string();
                            }
                        }
                        entries.push(entry_dir)
                    }
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
