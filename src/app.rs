use std::fs;

use crate::entry::{create_entry, create_entry_for_dir, Entry};
use crate::parser::{parse, Opti};
use crate::ref_command::*;
use glob::glob;

#[derive(Clone, Debug)]
pub struct App {
    pub entries: Vec<Entry>,
    pub name: &'static str,
    pub version: &'static str,
    pub options: Vec<Opti>

}

impl App    {
    pub fn init() -> Option<App>    {
        let mut app = App {
            entries: Vec::new(),
            name: &NAME,
            version: &VERSION,
            options: Vec::new()
        };

        let (options, values) = parse();
        let mut entries: Vec<Entry> = Vec::new();
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
        Some(app)
    }
}
