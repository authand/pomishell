use std::{fs::{self, DirEntry}, io};

pub fn get_input() -> String {
    let mut line: String = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line
}

pub fn parse_args(line: &str) -> Result<Vec<String>, String> {
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
        Err("[-] error in input".to_string())
    } else {
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