#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    // Uncomment this block to pass the first stage
    // print!("$ ");
    // io::stdout().flush().unwrap();

    // Wait for user input
    let stdin = io::stdin();
    let mut input = String::new();

    let path_env = std::env::var("PATH").unwrap();
   
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
            "type" => {
                let command = parts.next().unwrap_or("");
                if command.is_empty() {
                    println!("type: missing argument");
                } else {
                    
                    match command {
                        "echo" => println!("{} is a shell builtin", command),
                        "type" => println!("type is a shell builtin"),
                        "exit" => println!("exit is a shell builtin"),
                        _ => {
                            let path_env = std::env::var("PATH").unwrap();
                            let paths = path_env.split(':');
                            let mut found = false;

                            for path in paths {
                                let full_path = format!("{}/{}", path, command);
                                if std::path::Path::new(&full_path).exists() {
                                    println!("{} is {}", command, full_path);
                                    found = true;
                                    break;
                                }
                            }

                            if !found {
                                println!("{}: not found", command);
                            }
                        }
                    }
                }
            },
            _ => println!("{input}: command not found"),
        }
    }
}
