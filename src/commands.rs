use pomishell::file_size;
use std::fs;

pub const COMMANDS: [&str; 4] = ["cat", "cd", "ls", "exit"];

static DIR_COLOR: &str = "\x1b[38;5;131m";
static SIZE_COLOR: &str = "\x1b[38;5;166m";
static RESET: &str = "\x1b[0m";

pub fn ls_command(args: &[String]) -> Result<(), String> {
    println!();
    let path = if args.len() > 1 {
        args[1].as_str()
    } else {
        "."
    };

    match fs::canonicalize(path) {
        Ok(full_path) => {
            let display_path = full_path
                .to_str()
                .map(|s| s.strip_prefix(r"\\?\").unwrap_or(s))
                .unwrap_or_default();

            println!("{}Directory{}: {}\n", DIR_COLOR, RESET, display_path);
        }
        Err(e) => {
            return Err(format!("Couldn't get directory: {}", e));
        }
    }

    match fs::read_dir(path) {
    Ok(entries) => {
        for entry in entries {
            if let Ok(entry) = entry {
                if let Ok(file_name) = entry.file_name().into_string() {
                    let size: String = match file_size(entry) {
                        Ok(size) => size,
                        Err(e) => e.to_string(),
                    };

                    if size.is_empty() {
                        println!("{:>10}\t{}{}/{}", "", DIR_COLOR, file_name, RESET);
                    } else {
                        println!("{}{:>10}{}\t{}", SIZE_COLOR, size, RESET, file_name);
                    }
                }
            }
        }
        println!();
    }
    Err(e) => {
        return Err(format!("[-] ls: {}", e));
    }
}
    Ok(())
}

pub fn cd_command(args: &[String]) -> Result<(), String> {
    if args.len() < 2 {
        return Err("[-] expected argument to 'cd'".to_string());
    }

    match std::env::set_current_dir(&args[1]) {
        Ok(_) => Ok(()),
        Err(e) => {
            Err(format!("[-] error: {e}"))
        }
    }
}

pub fn cat_command(args:  &[String]) -> Result<(), String> {
    if args.len() < 2 {
        return Err("[-] expected argument to 'cat'".to_string());
    }

    match fs::read_to_string(&args[1]) {
        Ok(output) => {
            print!("{output}");
            Ok(())
        }
        Err(e) => {
            Err(format!("[-] error: {e}"))
        }
    }
}

pub fn run_external_command(args: &[String]) {
    if let Some((cmd, rest)) = args.split_first() {
        match std::process::Command::new(cmd).args(rest).status() {
            Ok(status) => {
                if !status.success() {
                    eprintln!("[-] process exited with status: {}", status)
                }
            }
            Err(e) => eprintln!("[-] failed to execute '{}': {}", cmd, e),
        }
    }
}