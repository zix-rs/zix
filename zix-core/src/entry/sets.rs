use std::collections::HashSet;

pub fn config_files() -> HashSet<&'static str> {
    let configs: HashSet<&str> = [
        ".conf",
        ".config",
        "Cargo.toml",
        "Cargo.lock",
        ".gitignore",
        ".env",
        ".editorconfig",
        "settings.json",
        "pyproject.toml",
        "package.json",
        "webpack.config.js",
        "tsconfig.json"
    ]
    .iter()
    .cloned()
    .collect();

    configs
}

pub fn is_config_file(filename: &str, configs: &HashSet<&str>) -> bool {
    configs.contains(filename) || configs.iter().any(|ext| filename.ends_with(ext))
}

pub fn get_language_extensions() -> HashSet<&'static str> {
    let extensions: HashSet<&'static str> = [
        "rs",   // Rust
        "py",   // Python
        "js",   // JavaScript
        "c",    // C
        "cpp",  // C++
        "java", // Java
        "go",   // Go
        "rb",   // Ruby
        "kt",   // Kotlin
        "swift",// Swift
        "ts",   // TypeScript
        "php",  // PHP
        "scala",// Scala
        "hs",   // Haskell
        "lua",  // Lua
        "html", // HTML
        "css",  // CSS
        "sh",   // Shell
        "pl",   // Perl
        "r"     // R
    ]
    .into_iter()
    .collect();

    extensions
}
