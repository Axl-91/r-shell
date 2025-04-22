use std::io::{self, stdout, Write};
use std::process::Command;

fn main() {
    loop {
        print!(">> ");
        _ = stdout().flush();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let command = input.trim();

        match Command::new(command).spawn() {
            Ok(mut child) => match child.wait() {
                Ok(exit_status) => println!(
                    "\n    Run [{:?}] exit_status: {:?}",
                    command,
                    exit_status.code().unwrap_or(-1)
                ),
                Err(err) => println!("Error on wait: {:?}", err.to_string()),
            },
            Err(err) => println!("Error: {:?}", err.to_string()),
        }
    }
}
