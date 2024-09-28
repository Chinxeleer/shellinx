use std::io::*;

fn main() {
    print!("shellinx> ");
    stdout().flush().unwrap();

    loop {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        println!("You typed: {}", input);
        print!("shellinx> ");
        stdout().flush().unwrap();
    }
}
