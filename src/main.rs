mod aegit;
mod constants;
mod paths;

use std::env;

fn main() {
    let command = env::args().nth(1);
    let args = env::args().skip(2);

    let repo_directory = match env::current_dir() {
        Ok(current_dir) =>  current_dir,
        Err(error) =>  panic!("{}", error)
    };

    match command {
        Some(cmd) => match cmd.as_str() {
            "init" => {
                if let Err(e) = aegit::init(&repo_directory) {
                    println!("Error: {}", e)
                }
            },
            "add" => {
                for file in args {
                    if let Err(e) = aegit::add(&repo_directory, file) {
                        println!("Error: {}", e)
                    }
                }
            }
            _ => println!("Wrong command!")
        },
        _ => println!("Command is missing!")
    }
}

