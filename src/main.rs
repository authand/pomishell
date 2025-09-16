mod commands;
use pomishell;

fn main() {
    println!("------------------'help' for list of commands------------------");
    loop {
        let path = std::env::current_dir().unwrap();
        print!("POMI {}> ", path.display());
        std::io::Write::flush(&mut std::io::stdout()).unwrap();

        let line = pomishell::get_input();
        let args = match pomishell::parse_args(&line) {
            Ok(v) => v,
            Err(e) =>  {
                eprintln!("[-] parse error: {}", e);
                continue;
            }
        };
        if args.is_empty() {
            continue;
        }

        let command_result = match args[0].as_str() {
            "help" => {
                for command in commands::COMMANDS.iter() {
                    println!("{}", command);
                }
                Ok(())
            }
            "cat" => {
                commands::cat_command(&args)
            }
            "cd" => {
                commands::cd_command(&args)
            }
            "ls" => {
                commands::ls_command(&args)
            }
            "exit" => {
                break;
            }
            _ => {
                commands::run_external_command(&args);
                Ok(())
            }
        };

        if let Err(e) = command_result {
            eprintln!("{}", e);
        }
    }
}