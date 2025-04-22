use std::env;
use std::io::{self, stdout, Write};
use std::path::{Path, PathBuf};
use std::process::Command;

fn run_cmd(command: &str, args: Vec<&str>, input: String) {
    let child_cmd = Command::new(command).args(args).spawn();

    match child_cmd {
        Ok(mut child) => match child.wait() {
            Ok(exit_status) => println!(
                "\n    \x1b[36mRun [{:?}] exit_status: {:?}\x1b[0m",
                input.trim(),
                exit_status.code().unwrap_or(-1)
            ),
            Err(err) => eprintln!("\x1b[31mError on wait: {}\x1b[0m", err),
        },
        Err(err) => eprintln!("\x1b[31mError: {}\x1b[0m", err),
    }
}

fn change_dir(dir: Vec<&str>, home: PathBuf) {
    let dir_option = dir.into_iter().peekable().peek().copied();

    let root = if let Some(new_dir) = dir_option {
        Path::new(new_dir)
    } else {
        Path::new(&home)
    };

    if let Err(err) = env::set_current_dir(root) {
        eprintln!("{}", err);
    }
}

fn main() {
    let home = dirs::home_dir().unwrap_or(PathBuf::from("/"));
    if let Err(err) = env::set_current_dir(&home) {
        eprintln!("{}", err)
    }
    loop {
        let current_dir = env::current_dir().unwrap();
        println!("\x1b[35m({})\x1b[0m", current_dir.display());
        print!(">> ");
        _ = stdout().flush();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let mut input_split = input.split_whitespace();

        let command = input_split.next().unwrap_or("");
        let args: Vec<&str> = input_split.collect();

        match command {
            "exit" => return,
            "pwd" => println!("{}", current_dir.display()),
            "cd" => change_dir(args, home.clone()),
            command => run_cmd(command, args, input.clone()),
        }
    }
}
