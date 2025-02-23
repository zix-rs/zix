use std::fs;

use zix_core::entry::{
    create,
    create::Opti
};
use zix_core::entry::{Entry};

use zix_utils::parser::parser;
use crate::ref_command::{help, version, NAME, VERSION};

#[cfg(windows)]
use glob::glob;

use std::env;


pub fn parse() -> (Vec<String>, Vec<String>)  {
    let args = env::args().skip(1);

    let mut options: Vec<String> = Vec::new();
    let mut values: Vec<String> = Vec::new();

    for arg in args {
        if arg.starts_with("--") {
            options.push(arg);
        } else if arg.starts_with('-') && arg.len() > 1 {
            options.extend(arg[1..].chars().map(|ch| format!("-{}", ch)));
        } else {
            values.push(arg);
        }
    }

    if values.is_empty() {
        values.push(".".to_string());
    }

    (options, values)
}


#[derive(Clone, Debug)]
pub struct App {
    pub entries: Vec<Vec<Entry>>,
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

        if let Some((_c, options, values)) = parser(
            false,
            ".",
            "-g",
            ""
        ) {

            for op in options   {
                match op.as_str()    {
                    "--help" | "-?" => { help(); return None},
                    "--headers" | "-h" => app.options.push(Opti::Headers),
                    "--version" | "-v" => { version(); return None},
                    "--all" | "-a" => app.options.push(Opti::All),
                    "--list" | "-l" => app.options.push(Opti::List),
                    "--tree" | "-t" => app.options.push(Opti::Tree),
                    "--grid" | "-g" => app.options.push(Opti::Grid),
                    _ => {
                        println!(
                            "'{}' is not a valid option\nType 'zx --help' for more information.",
                            op
                        );
                        return None
                    }
                }
            }


            #[cfg(unix)] {
                for val in values.iter() {
                    let mut entries: Vec<Entry> = Vec::new();

                    match fs::metadata(val) {
                        Ok(metadata) => {
                            if metadata.is_file() {
                                let path = std::path::PathBuf::from(val);
                                if let Some(entry) = create::filter_dir(&path) {
                                    entries.push(entry);
                                }
                            } else if metadata.is_dir() {
                                if let Ok(dir) = fs::read_dir(val) {
                                    entries.extend(
                                        dir
                                            .filter_map(Result::ok)
                                            .filter_map(|path| create::dir(&path, &app.options))
                                    );
                                } else {
                                    println!("Cannot read directory: {}", val);
                                }
                            }
                        },
                        Err(_) => println!("Cannot access path: {}", val)
                    }

                    app.entries.push(entries)
                }
            }

            #[cfg(windows)] {
            for val in values.iter()   {
                let mut entries: Vec<Entry> = Vec::new();

                    if val.contains('*')    {
                        if let Ok(paths) = glob(&val) {
                            entries.extend(
                                paths
                                    .filter_map(Result::ok)
                                    .filter_map(|path| create::filter_dir(&path))
                            );
                        } else {
                            println!("Error interpreting the pattern: {}", val);
                        }
                    } else {
                        if let Ok(dir) = fs::read_dir::<&String>(&val)   {
                            entries.extend(
                                dir
                                        .filter_map(Result::ok)
                                        .filter_map(
                                            |path|
                                            create::dir(&path, &app.options)
                                        )
                            );
                    } else {
                        continue;
                    };
                    }
                    app.entries.push(entries);
                }
            }

            return Some(app)
        }

        Some(app)
    }
}
