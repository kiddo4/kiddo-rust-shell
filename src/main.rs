#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    // Uncomment this block to pass the first stage
    // print!("$ ");
    // io::stdout().flush().unwrap();

    // Wait for user input
    let stdin = io::stdin();
    let mut input = String::new();
   

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        input.clear();
        
        stdin.read_line(&mut input).unwrap();
        input.pop();
        println!("{}: command not found", input);
    }
}
