use pomishell::file_size;
use std::{
    fs::{self, File, OpenOptions},
    io::Write,
    process::{Command, Stdio},
};

pub const COMMANDS: [&str; 11] = [
    "cat", "cd", "clear", "echo", "exit", "help", "history", "ls", "mkdir", "pwd", "touch",
];

static DIR_COLOR: &str = "\x1b[38;5;131m";
static SIZE_COLOR: &str = "\x1b[38;5;166m";
static RESET: &str = "\x1b[0m";

struct CommandLine {
    commands: Vec<Vec<String>>,
    input: Option<String>,
    output: Option<(String, bool)>,
}

pub fn run_command(args: &[String], history: &[String]) -> Result<bool, String> {
    let line = command_line(args)?;

    if line.commands.len() == 1 {
        match line.commands[0][0].as_str() {
            "cd" => {
                cd_command(&line.commands[0])?;
                return Ok(true);
            }
            "exit" => return Ok(false),
            _ => {}
        }
    } else if line
        .commands
        .iter()
        .any(|cmd| matches!(cmd.first().map(|s| s.as_str()), Some("cd") | Some("exit")))
    {
        return Err("[-] cd and exit can't be used in pipes".to_string());
    }

    if line.commands.len() == 1
        && line.input.is_none()
        && line.output.is_none()
        && !is_builtin(&line.commands[0])
    {
        run_external_command(&line.commands[0])?;
        return Ok(true);
    }

    let mut input = match line.input {
        Some(path) => Some(fs::read(&path).map_err(|e| format!("[-] input: {e}"))?),
        None => None,
    };

    for command in line.commands.iter() {
        input = Some(run_one(command, input.take(), history)?);
    }

    let output = input.unwrap_or_default();
    if let Some((path, append)) = line.output {
        write_output(&path, append, &output)?;
    } else {
        std::io::stdout()
            .write_all(&output)
            .map_err(|e| format!("[-] output: {e}"))?;
        std::io::stdout()
            .flush()
            .map_err(|e| format!("[-] output: {e}"))?;
    }

    Ok(true)
}

fn command_line(args: &[String]) -> Result<CommandLine, String> {
    let mut commands = vec![Vec::new()];
    let mut input = None;
    let mut output = None;
    let mut i = 0;

    while i < args.len() {
        match args[i].as_str() {
            "|" => {
                if commands.last().unwrap().is_empty() {
                    return Err("[-] empty pipe".to_string());
                }
                commands.push(Vec::new());
            }
            "<" => {
                let path = redirect_path(args, i)?;
                if input.replace(path).is_some() {
                    return Err("[-] multiple input redirects".to_string());
                }
                i += 1;
            }
            ">" | ">>" => {
                let append = args[i] == ">>";
                let path = redirect_path(args, i)?;
                if output.replace((path, append)).is_some() {
                    return Err("[-] multiple output redirects".to_string());
                }
                i += 1;
            }
            _ => commands.last_mut().unwrap().push(args[i].clone()),
        }
        i += 1;
    }

    if commands.last().unwrap().is_empty() {
        return Err("[-] empty pipe".to_string());
    }

    Ok(CommandLine {
        commands,
        input,
        output,
    })
}

fn redirect_path(args: &[String], i: usize) -> Result<String, String> {
    match args.get(i + 1) {
        Some(path) if !is_operator(path) => Ok(path.clone()),
        _ => Err(format!("[-] expected file after '{}'", args[i])),
    }
}

fn is_operator(arg: &str) -> bool {
    matches!(arg, "|" | "<" | ">" | ">>")
}

fn is_builtin(args: &[String]) -> bool {
    matches!(
        args.first().map(|s| s.as_str()),
        Some("cat")
            | Some("clear")
            | Some("echo")
            | Some("help")
            | Some("history")
            | Some("ls")
            | Some("mkdir")
            | Some("pwd")
            | Some("touch")
    )
}

fn run_one(args: &[String], input: Option<Vec<u8>>, history: &[String]) -> Result<Vec<u8>, String> {
    match args[0].as_str() {
        "cat" => cat_output(args, input),
        "clear" => Ok(b"\x1b[2J\x1b[1;1H".to_vec()),
        "echo" => Ok(format!("{}\n", args[1..].join(" ")).into_bytes()),
        "help" => Ok(format!("{}\n", COMMANDS.join("\n")).into_bytes()),
        "history" => Ok(history_output(history).into_bytes()),
        "ls" => Ok(ls_output(args)?.into_bytes()),
        "mkdir" => {
            mkdir_command(args)?;
            Ok(Vec::new())
        }
        "pwd" => Ok(format!("{}\n", std::env::current_dir().unwrap().display()).into_bytes()),
        "touch" => {
            touch_command(args)?;
            Ok(Vec::new())
        }
        _ => run_external_capture(args, input),
    }
}

fn ls_output(args: &[String]) -> Result<String, String> {
    let mut output = String::new();
    output.push('\n');

    let path = if args.len() > 1 {
        args[1].as_str()
    } else {
        "."
    };

    // patch windows file path having some weird artifact thing
    match fs::canonicalize(path) {
        Ok(full_path) => {
            let display_path = full_path
                .to_str()
                .map(|s| s.strip_prefix(r"\\?\").unwrap_or(s))
                .unwrap_or_default();

            output.push_str(&format!(
                "{}Directory{}: {}\n\n",
                DIR_COLOR, RESET, display_path
            ));
        }
        Err(e) => {
            return Err(format!("Couldn't get directory: {}", e));
        }
    }

    match fs::read_dir(path) {
        Ok(entries) => {
            for entry in entries.flatten() {
                if let Ok(file_name) = entry.file_name().into_string() {
                    let size: String = match file_size(entry) {
                        Ok(size) => size,
                        Err(e) => e.to_string(),
                    };

                    if size.is_empty() {
                        output.push_str(&format!(
                            "{:>10}\t{}{}/{}\n",
                            "", DIR_COLOR, file_name, RESET
                        ));
                    } else {
                        output.push_str(&format!(
                            "{}{:>10}{}\t{}\n",
                            SIZE_COLOR, size, RESET, file_name
                        ));
                    }
                }
            }
            output.push('\n');
        }
        Err(e) => {
            return Err(format!("[-] ls: {}", e));
        }
    }

    Ok(output)
}

pub fn cd_command(args: &[String]) -> Result<(), String> {
    if args.len() < 2 {
        return Err("[-] expected argument to 'cd'".to_string());
    }

    match std::env::set_current_dir(&args[1]) {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("[-] error: {e}")),
    }
}

fn cat_output(args: &[String], input: Option<Vec<u8>>) -> Result<Vec<u8>, String> {
    if args.len() < 2 {
        return input.ok_or("[-] expected argument to 'cat'".to_string());
    }

    let mut output = Vec::new();
    for path in &args[1..] {
        let mut bytes = fs::read(path).map_err(|e| format!("[-] error: {e}"))?;
        output.append(&mut bytes);
    }

    Ok(output)
}

fn mkdir_command(args: &[String]) -> Result<(), String> {
    if args.len() < 2 {
        return Err("[-] expected argument to 'mkdir'".to_string());
    }

    for path in &args[1..] {
        fs::create_dir_all(path).map_err(|e| format!("[-] mkdir: {e}"))?;
    }

    Ok(())
}

fn touch_command(args: &[String]) -> Result<(), String> {
    if args.len() < 2 {
        return Err("[-] expected argument to 'touch'".to_string());
    }

    for path in &args[1..] {
        OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)
            .map_err(|e| format!("[-] touch: {e}"))?;
    }

    Ok(())
}

fn history_output(history: &[String]) -> String {
    let mut output = String::new();
    for (i, line) in history.iter().enumerate() {
        output.push_str(&format!("{:>4}  {}\n", i + 1, line));
    }
    output
}

fn write_output(path: &str, append: bool, output: &[u8]) -> Result<(), String> {
    let mut file = if append {
        OpenOptions::new().create(true).append(true).open(path)
    } else {
        File::create(path)
    }
    .map_err(|e| format!("[-] redirect: {e}"))?;

    file.write_all(output)
        .map_err(|e| format!("[-] redirect: {e}"))
}

fn run_external_capture(args: &[String], input: Option<Vec<u8>>) -> Result<Vec<u8>, String> {
    if let Some((cmd, rest)) = args.split_first() {
        let mut command = Command::new(cmd);
        command
            .args(rest)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        if input.is_some() {
            command.stdin(Stdio::piped());
        }

        let mut child = command
            .spawn()
            .map_err(|e| format!("[-] failed to execute '{}': {}", cmd, e))?;

        if let Some(input) = input {
            if let Some(mut stdin) = child.stdin.take() {
                stdin
                    .write_all(&input)
                    .map_err(|e| format!("[-] stdin: {e}"))?;
            }
        }

        let output = child
            .wait_with_output()
            .map_err(|e| format!("[-] failed to wait for '{}': {}", cmd, e))?;

        if !output.stderr.is_empty() {
            eprint!("{}", String::from_utf8_lossy(&output.stderr));
        }

        if !output.status.success() {
            eprintln!("[-] process exited with status: {}", output.status)
        }

        Ok(output.stdout)
    } else {
        Ok(Vec::new())
    }
}

pub fn run_external_command(args: &[String]) -> Result<(), String> {
    if let Some((cmd, rest)) = args.split_first() {
        match Command::new(cmd).args(rest).status() {
            Ok(status) => {
                if !status.success() {
                    eprintln!("[-] process exited with status: {}", status)
                }
                Ok(())
            }
            Err(e) => Err(format!("[-] failed to execute '{}': {}", cmd, e)),
        }
    } else {
        Ok(())
    }
}
