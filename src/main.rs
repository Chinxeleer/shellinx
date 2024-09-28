use std::io::*;
fn exit_function(input: &str) -> bool {
    if input.trim() == "exit" {
        println!("Exiting shellinx...");
        return true;
    }

    false
}

fn main() {
    loop {
        print!("shellinx> ");
        stdout().flush().unwrap();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        if exit_function(&input) {
            break;
        }
    }
}
