use pomishell::file_size;
use std::fs;

pub const COMMANDS: [&str; 4] = ["cat", "cd", "ls", "exit"];


pub fn ls_command(args: &[String]) -> bool {
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

            println!("Directory: {}\n", display_path);
        }
        Err(e) => {
            eprintln!("Couldn't get directory: {}", e);
            return true;
        }
    }

    match fs::read_dir(path) {
        Ok(entries) => {
            for entry in entries {
                if let Ok(entry) = entry {
                    if let Ok(file_name) = entry.file_name().into_string() {
                        let size: String = match file_size(entry) {
                            Ok(size) => size,
                            Err(e) => e.to_string()
                        }; 
                        println!("{}\t{}", size, file_name);
                    }
                }
            }
            println!();
        }
        Err(e) => {
            eprintln!("[-] ls: {}", e);
        }
    }

    true
}

pub fn cd_command(args: &[String]) -> bool {
    if args.len() < 2 {
        eprintln!("[-] expected argument to 'cd'");
        return true;
    }

    match std::env::set_current_dir(&args[1]) {
        Ok(_) => true,
        Err(e) => {
            eprintln!("[-] error: {e}");
            true
        }
    }
}

pub fn cat_command(args:  &[String]) -> bool {
    if args.len() < 2 {
        eprintln!("[-] expected argument to 'cat'");
        return true;
    }

    match fs::read_to_string(&args[1]) {
        Ok(output) => {
            print!("{output}");
            true
        }
        Err(e) => {
            eprintln!("[-] error: {e}");
            true
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