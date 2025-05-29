use std::fs;

fn get_input() -> String {
    let mut line: String = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line
}

fn parse_args(line: &str) -> Result<Vec<String>, String> {
    let mut args = Vec::new();
    let mut in_quotes = false;
    let mut current = String::new();

    for token in line.trim().split_whitespace() {
        if in_quotes {
            current.push(' ');
            current.push_str(token);
            if token.ends_with('\"') {
                in_quotes = false;
                args.push(current.trim_matches('\"').to_string());
                current = String::new();
            }
        } else if token.starts_with('\"') {
            if token.ends_with('\"') && token.len() > 1 {
                args.push(token.trim_matches('\"').to_string());
            } else {
                in_quotes = true;
                current.push_str(token);
            }
        } else {
            args.push(token.to_string());
        }
    }

    if in_quotes {
        Err("error in input".to_string())
    } else {
        Ok(args)
    }
}

fn cd_command(args: &[String]) -> bool {
    if args.len() < 2 {
        eprintln!("expected arguement to 'cd'");
        return true;
    }

    match std::env::set_current_dir(&args[1]) {
        Ok(_) => true,
        Err(e) => {
            eprintln!("error: {e}");
            true
        }
    }
}

fn ls_command(args: &[String]) -> bool {
    let path = if args.len() > 1 {
        args[1].as_str()
    } else {
        "."
    };

    match fs::read_dir(path) {
        Ok(entries) => {
            for entry in entries {
                if let Ok(entry) = entry {
                    if let Ok(file_name) = entry.file_name().into_string() {
                        println!("{}", file_name);
                    }
                }
            }
            println!();
        }
        Err(e) => {
            eprintln!("ls: {}", e);
        }
    }

    true
}

fn cat_command() {
    todo!()
}

fn main() {
    loop {
        let path = std::env::current_dir().unwrap();
        print!("pomi:{}> ", path.display());
        std::io::Write::flush(&mut std::io::stdout()).unwrap();

        let line = get_input();
        let args = match parse_args(&line) {
            Ok(v) => v,
            Err(e) =>  {
                eprintln!("parse error: {}", e);
                continue;
            }
        };
        if args.is_empty() {
            continue;
        }

        match args[0].as_str() {
            "cd" => {
                cd_command(&args);
            }
            "ls" => {
                ls_command(&args);
            }
            "exit" => {
                break;
            }
            _ => {
                eprintln!("unknown command: {}", args[0])
            }

        }
    }
}