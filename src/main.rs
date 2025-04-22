use std::io::{self, stdout, Write};
use std::process::Command;

fn main() {
    loop {
        print!(">> ");
        _ = stdout().flush();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let mut input_split = input.split_whitespace();

        let command = input_split.next().unwrap_or("");
        let args = input_split;

        if command == "exit" {
            return;
        }

        match Command::new(command).args(args).spawn() {
            Ok(mut child) => match child.wait() {
                Ok(exit_status) => println!(
                    "\n    \x1b[36mRun [{:?}] exit_status: {:?}\x1b[0m",
                    input.trim(),
                    exit_status.code().unwrap_or(-1)
                ),
                Err(err) => println!("\x1b[31mError on wait: {:?}\x1b[0m", err.to_string()),
            },
            Err(err) => println!("\x1b[31mError: {:?}\x1b[0m", err.to_string()),
        }
    }
}
