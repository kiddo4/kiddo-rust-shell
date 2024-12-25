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

        
        let mut parts = input.split(' ');
        match parts.next().unwrap() {
            "exit" => std::process::exit(parts.next().and_then(|x| x.parse().ok()).unwrap_or(0)),
            "echo" => println!("{}", parts.collect::<Vec<&str>>().join(" ")),
            _ => println!("{input}: command not found"),
        }
    }
}
