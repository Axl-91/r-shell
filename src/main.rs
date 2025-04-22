use std::env;
use std::io::{self, stdout, Write};
use std::path::{Path, PathBuf};
use std::process::Command;

mod defs;

fn run_cmd(command: &str, args: Vec<&str>, input: String) {
    let child_cmd = Command::new(command).args(args).spawn();

    match child_cmd {
        Ok(mut child) => match child.wait() {
            Ok(exit_status) => println!(
                "\n    {}Run [{:?}] exit_status: {:?}{}",
                defs::BLU,
                input.trim(),
                exit_status.code().unwrap_or(-1),
                defs::RESET
            ),
            Err(err) => eprintln!("{}Error on wait: {}{}", defs::RED, err, defs::RESET),
        },
        Err(err) => eprintln!("{}Error: {}{}", defs::RED, err, defs::RESET),
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
        eprintln!("{}Error: {}{}", defs::RED, err, defs::RESET);
    }
}

fn main() {
    let home = dirs::home_dir().unwrap_or(PathBuf::from("/"));
    if let Err(err) = env::set_current_dir(&home) {
        eprintln!("{}Error: {}{}", defs::RED, err, defs::RESET);
    }
    loop {
        let current_dir = env::current_dir().unwrap();
        println!("{}({}){}", defs::MAG, current_dir.display(), defs::RESET);
        print!(">> ");
        _ = stdout().flush();

        let mut input = String::new();
        let bytes_read = io::stdin().read_line(&mut input).unwrap();

        if bytes_read == 0 {
            return;
        }

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
