mod commands;

fn main() {
    println!("------------------'help' for list of commands------------------");
    let mut history = Vec::new();

    loop {
        let path = std::env::current_dir().unwrap();
        print!("POMI {}> ", path.display());
        std::io::Write::flush(&mut std::io::stdout()).unwrap();

        let line = pomishell::get_input();
        let line = line.trim_end().to_string();
        let args = match pomishell::parse_args(&line) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("[-] parse error: {}", e);
                continue;
            }
        };

        if args.is_empty() {
            continue;
        }

        history.push(line);
        match commands::run_command(&args, &history) {
            Ok(true) => {}
            Ok(false) => break,
            Err(e) => eprintln!("{}", e),
        }
    }
}
