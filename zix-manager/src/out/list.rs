use std::{fs, io};
use zix_core::plugin::Plugin;
use libloading::{Library, Symbol};

pub fn list() {
    if let Some(home)  = dirs::home_dir() {
        let zixf = home.join(".zix");
        let plugins = zixf.join("plugins");

        if let Ok(entries) = fs::read_dir(plugins) {
            let plugins_path = entries
                .map(|res| res.map(|e| e.path()))
                .collect::<Result<Vec<_>, io::Error>>();

            if let Ok(plugins) = plugins_path {
                println!("Installed Plugins: \n");
                for plugin in plugins {
                    unsafe {
                        let lib = Library::new(plugin)
                            .expect("Failed to load plugin");
                        let constructor: Symbol<unsafe extern fn() -> *mut dyn Plugin> = lib.get(b"create_plugin").expect("Failed to find symbol");

                        let boxed_raw = constructor();
                        let boxed = Box::from_raw(boxed_raw);
                        let info = boxed.info();

                        print!("{}\t", info.name);
                        if let Some(desc) = info.desc {
                            print!("{}\n", desc)
                        } else {
                            print!("Dont have a description\n")
                        }

                    }
                }
            }
        }
    }
}
