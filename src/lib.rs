use std::{
    fs::{self, DirEntry},
    io,
};

pub fn get_input() -> String {
    let mut line: String = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line
}

pub fn parse_args(line: &str) -> Result<Vec<String>, String> {
    let mut args = Vec::new();
    let mut in_quotes = false;
    let mut current = String::new();

    let mut chars = line.trim().chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            '\"' => in_quotes = !in_quotes,
            c if c.is_whitespace() && !in_quotes => {
                if !current.is_empty() {
                    args.push(current);
                    current = String::new();
                }
            }
            '|' | '<' if !in_quotes => {
                if !current.is_empty() {
                    args.push(current);
                    current = String::new();
                }
                args.push(c.to_string());
            }
            '>' if !in_quotes => {
                if !current.is_empty() {
                    args.push(current);
                    current = String::new();
                }

                if chars.peek() == Some(&'>') {
                    chars.next();
                    args.push(">>".to_string());
                } else {
                    args.push(">".to_string());
                }
            }
            _ => current.push(c),
        }
    }

    if in_quotes {
        Err("[-] error in input".to_string())
    } else {
        if !current.is_empty() {
            args.push(current);
        }
        Ok(args)
    }
}

pub fn file_size(entry: DirEntry) -> io::Result<String> {
    let metadata = fs::metadata(entry.path())?;
    // println!("{metadata:?}");

    if metadata.is_dir() {
        Ok(String::new())
    } else {
        Ok(metadata.len().to_string())
    }
}
