use std::fs;

use zix_core::entry::create;
use zix_core::entry::{Entry, create::filter_entries};

use zix_utils::parser::{self, parser};
use crate::ref_command::{help, version, NAME, VERSION};

#[cfg(windows)]
use glob::glob;

use zix_core::entry::create::FilterOptions;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Opti   {
    All,
    List,
    Help,
    Version,
    Headers,
    Icons,
    Tree,
    Grid
}

impl FilterOptions for Opti {
    fn should_include_hidden(&self) -> bool {
        matches!(self, Opti::All)
    }

    fn should_show_icons(&self) -> bool {
        matches!(self, Opti::Icons)
    }

    fn should_use_tree_view(&self) -> bool {
        matches!(self, Opti::Tree)
    }

    fn should_use_list_view(&self) -> bool {
        matches!(self, Opti::List)
    }

    fn should_use_grid_view(&self) -> bool {
        matches!(self, Opti::Grid)
    }
}

#[derive(Clone, Debug)]
pub struct App {
    pub entries: Vec<Vec<Entry>>,
    pub name: &'static str,
    pub version: &'static str,
    pub options: Vec<Opti>,
}

impl App {
    pub fn init() -> Option<App> {
        let mut app = App {
            entries: Vec::new(),
            name: &NAME,
            version: &VERSION,
            options: Vec::new()
        };
        if let Some((
                _command,
                options,
                values
            )) = parser(
                false,
                ".",
                "--grid",
                "")
        {
            for op in options {
                match op.as_str() {
                    "--help" | "-?" => {
                        help();
                        return None;
                    }
                    "--headers" | "-h" => app.options.push(Opti::Headers),
                    "--version" | "-v" => {
                        version();
                        return None;
                    }
                    "--all" | "-a" => app.options.push(Opti::All),
                    "--list" | "-l" => app.options.push(Opti::List),
                    "--tree" | "-t" => app.options.push(Opti::Tree),
                    "--grid" | "-g" => app.options.push(Opti::Grid),
                    _ => {
                        println!(
                            "'{}' is not a valid option\nType 'zx --help' for more information.",
                            op
                        );
                        return None;
                    }
                }
            }

            #[cfg(unix)]
            {
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
                                    let unfiltered_entries: Vec<Entry> = dir
                                        .filter_map(Result::ok)
                                        .filter_map(|path| create::dir(&path, &app.options[0]))
                                        .collect();
                                    entries.extend(filter_entries(&unfiltered_entries, &app.options[0]));
                                } else {
                                    println!("Cannot read directory: {}", val);
                                }
                            }
                        }
                        Err(_) => println!("Cannot access path: {}", val),
                    }

                    app.entries.push(entries);
                }
            }

            #[cfg(windows)]
            {
                for val in values.iter() {
                    let mut entries: Vec<Entry> = Vec::new();

                    if val.contains('*') {
                        if let Ok(paths) = glob(&val) {
                            entries.extend(
                                paths
                                    .filter_map(Result::ok)
                                    .filter_map(|path| create::filter_dir(&path)),
                            );
                        } else {
                            println!("Error interpreting the pattern: {}", val);
                        }
                    } else {
                        if let Ok(dir) = fs::read_dir::<&String>(&val) {
                            let unfiltered_entries: Vec<Entry> = dir
                                .filter_map(Result::ok)
                                .filter_map(|path| create::dir(&path, &app.options[0]))
                                .collect();
                            if app.options.is_empty() {
                                entries.extend(unfiltered_entries);
                            } else {
                                entries.extend(filter_entries(&unfiltered_entries, &app.options[0]));
                            }
                        } else {
                            continue;
                        };
                    }
                    app.entries.push(entries);
                }
            }
        }

        Some(app)
    }
}
