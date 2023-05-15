use std::{
    env,
    fs,
    path,
    process,
};

use colored::Colorize; // Something like this - No internet atm to confirm

#[derive(Clone, Copy, Debug)]
enum FormatTypes {
    Normal,
    Number,
    SingleQuoteString,
    DoubleQuoteString,
    Strikethrough,
    Highlight,
    Italicize,
    Bold,
}

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

fn apply_styles(file_contents: String) -> String {
    let mut styled_content = String::new();

    // Loop through each line of the contents
    // If we encounter a specific character, set a variable, keep going
    styled_content = file_contents.lines().map(|row| {
        let mut previous_style_char = false;
        let mut styled_row: String = String::new();
        // If our row is `Hello, World!`
        // We should begin styling at the first '`', and end on the last '`'
        styled_row = row.chars().map(|character| {
            match character {
                '~' => {
                    println!("Squiggle!");
                },
                '*' => {
                    println!("Asterisk!");
                },
                '_' => {
                    println!("UnderScore!");
                },
                '`' => {
                    println!("BackTick!");
                },
                '\'' => {
                    println!("Single Quote!");
                },
                '"' => {
                    println!("Double Quote!");
                },
                _ => {
                    if character.is_digit(10) {
                        println!("Character was number");
                    } else {
                        println!("Character is normal");
                    }
                }, 
            }
            
            character
        }).collect();

        styled_row
    }).collect();

    styled_content
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

    let file_contents = fs::read_to_string(&file_path).unwrap_or_else(|err| {
        eprintln!("ERROR: Could not read file contents to string {file_path}: {err}");
        process::exit(1)
    });

    let styled_content: String = apply_styles(file_contents);
    
    println!("{styled_content}");
}
