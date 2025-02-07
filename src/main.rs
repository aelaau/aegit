use std::env;
use std::fs;
use std::io;

const GIT_DIRECTORY: &str = ".aegit";

fn main() {
    let command = env::args().nth(1);
    // let args = env::args().skip(2);

    let directory = match env::current_dir() {
        Ok(current_dir) =>  current_dir,
        Err(error) =>  panic!("{}", error)
    };

    match command {
        Some(cmd) => match cmd.as_str() {
            "init" => {
                if let Err(e) = init(directory) {
                    println!("Error: {}", e)
                }
            },
            _ => println!("Wrong command!")
        },
        _ => println!("Command is missing!")
    }
}

fn init(directory: std::path::PathBuf) -> io::Result<()> {
    let git_directory = directory.join(GIT_DIRECTORY);

    if git_directory.exists() {
        return Err(io::Error::new(io::ErrorKind::AlreadyExists, "Git is already initialized"));
    }
    fs::create_dir(git_directory.as_path())?;

    println!("Git is initialized in {}", directory.display());
    Ok(())
}