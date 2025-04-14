use std::{fs, io::Write};

pub fn init() {
    let default_config = r#"
    # Zix Config
    plugin_dir = "./plugins"
    "#;
    if let Some(home) = dirs::home_dir() {
        let zix_dir = home.join(".zix");
        if !zix_dir.exists() {
            fs::create_dir(&zix_dir).expect("No se pudo crear ~/.zix");
            println!("Creating: {:?}", zix_dir);
        }

        let plugins = zix_dir.join("plugins");
        if !plugins.exists() {
            fs::create_dir(&plugins).expect("No se pudo crear plugins/");
            println!("Creating: {:?}", plugins);
        }

        let config = zix_dir.join(".config.toml");
        if !config.exists() {
            let mut file = fs::File::create(&config).expect(".config.toml");

            file.write_all(default_config.as_bytes()).expect(" .config.toml");
            println!("Creado: {:?}", config);
        } else {
            println!("Archivo de configuración ya existe: {:?}", config);
        }

    } else {
        println!("Home doesnt exist")
    }
}
