use std::fs::{self, Metadata, Permissions};

use colored::Colorize;

pub fn is_executable(filename: &str, _metadata: &fs::Metadata) -> bool {
    if filename.ends_with(".exe") || filename.ends_with(".bat") || filename.ends_with(".cmd") {
        return true;
    }
    false
}

pub fn format_file_size(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;
    let str_out = if bytes >= GB {
        let b = format!("{:.1}", bytes as f64 / GB as f64);
        format!("{} {}", b.normal(), "GB".bright_yellow())
    } else if bytes >= MB {
        let b = format!("{:.1}", bytes as f64 / MB as f64);
        format!("{} {}", b.normal(), "MB".bright_cyan())
    } else if bytes >= KB {
        let b = format!("{:.1}", bytes as f64 / KB as f64);
        format!("{} {}", b.normal(), "KB".bright_magenta())
    } else if bytes > 0 {
        format!("{} {}", bytes.to_string().normal(), "B".bright_red())
    } else  {
        format!("{}", "-".bright_white())
    };

    str_out
}


pub fn entry_mode(meta: Metadata, perm: Permissions) -> String   {
    let mut mode = String::new();
    if cfg!(target_os = "windows")  {
        mode = format!(
            "{}{}{}",
            if meta.is_dir() {
                "d".bright_blue()
            } else {
                "-".normal()
            },
            if meta.is_file() {
                "a".bright_black()
            } else {
                "-".normal()
            },
            if perm.readonly() {
                "r-".bright_yellow()
            } else {
                "rw".normal()
            }
        );
    } else  {
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;

            let permissions = perm.mode();
            mode = format!(
                "{}{}{}{}{}{}{}{}{}",
                if meta.is_dir() {
                    "d".bright_blue()
                } else {
                    "-".normal()
                },
                if permissions & 0o400 != 0 {
                    "r".bright_green()
                } else {
                    "-".normal()
                },
                if permissions & 0o200 != 0 {
                    "w".bright_yellow()
                } else {
                    "-".normal()
                },
                if permissions & 0o100 != 0 {
                    "x".bright_red()
                } else {
                    "-".normal()
                },
                if permissions & 0o040 != 0 {
                    "r".bright_green()
                } else {
                    "-".normal()
                },
                if permissions & 0o020 != 0 {
                    "w".bright_yellow()
                } else {
                    "-".normal()
                },
                if permissions & 0o010 != 0 {
                    "x".bright_red()
                } else {
                    "-".normal()
                },
                if permissions & 0o004 != 0 {
                    "r".bright_green()
                } else {
                    "-".normal()
                },
                if permissions & 0o002 != 0 {
                    "w".bright_yellow()
                } else {
                    "-".normal()
                }
            );
        }
    }
    return mode
}
