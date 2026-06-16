# POMISHELL

This is a basic shell I wrote in Rust as a learning experience. It supports a few built-in commands and can execute external programs.

## Features

- Command execution with arguments
- Built-in commands like `cat`, `ls`, `cd`, `pwd`, `echo`, `mkdir`, `touch`, `clear`, `history` and external commands
- Input/output redirection with `<`, `>` and `>>`
- Piping with `|`
- Minimal error handling
- Basic command parsing
- Command history with `history`

## Note

`rm` command is not yet implemented

## Building

Requires [Rust](https://www.rust-lang.org/tools/install). Clone the repository and run:

```bash
cargo build --release
```

The binary will be in `target/release/`.

## Usage

Run the shell:

```bash
cargo run
```

Or, after building:

```bash
./target/release/pomishell
```

Type commands and press enter. Use `exit` to quit.
