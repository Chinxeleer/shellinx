use std::process::{self, Command};

pub fn exit_command(input: &[&str]) {
    if input[0].trim() == "exit" {
        println!("Exiting shellinx...");
        process::exit(0);
    }
}

pub fn cd_command(input: &[&str]) {
    if input[0].trim() == "cd" {
        println!("cd command");
        Command::new("cd")
            .arg(input[1..].join(" "))
            .spawn()
            .expect("Failed to execute command");
    }
}
