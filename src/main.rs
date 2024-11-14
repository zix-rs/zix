pub mod app;
pub mod ref_command;
pub mod parser;
use app::App;
use parser::Opti;

fn main() {
    if let Some(app) = App::init() {
        if app.dirs.is_empty()  {
            for entry in app.entries    {
                if app.options.contains(&Opti::List)  {
                    println!(
                        "{:<6} \t {:<19} {:>8} {}",
                        entry.mode, entry.last_modified, entry.lenght, entry.name
                    );
                } else {
                    print!("{} \t", entry.name)
                }
            }
            print!("a")
        } else {
            for dir in &app.dirs {
                for entry in &app.entries    {
                    if dir == &entry.father  {
                        if app.options.contains(&Opti::List)  {
                            println!(
                                "{:<6} \t {:<19} {:>8} {}",
                                entry.mode, entry.last_modified, entry.lenght, entry.name
                            );
                        } else {
                            print!("{} \t", entry.name)
                        }
                    }

                }
                println!("")
            }
        }
    } else {
        return;
    }
}
