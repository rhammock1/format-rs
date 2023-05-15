use std::{
    env,
    fs,
    path,
    process,
};

fn parse_args(arguments: &Vec<String>) -> Result<String, &'static str> {
    if arguments.len() == 1 {
        return Err("No arguments provided");
    }

    if arguments.len() > 2 {
        return Err("Too many arguments provided.");
    }
    let mut file_path = String::new();

    let path = path::Path::new(&arguments[1]);
    if path.exists() {
        file_path = arguments[1].clone().to_string();
    } else {
        return Err("Path does not exist");
    }

    Ok(file_path)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path: String = parse_args(&args).unwrap_or_else(|err| {
        eprintln!("ERROR: Could not parse program arguments {args:?}: {err}");
        process::exit(1)
    });

    if file_path == "" {
        println!("No file path provided");
        process::exit(0);
    }
    println!("File Path: {file_path}");    
}
