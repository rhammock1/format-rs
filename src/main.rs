use std::{
    io::{self, BufRead},
    env,
    fs,
    path,
    process,
};

use colored::Colorize;

#[derive(Clone, Copy, Debug)]
enum FormatType {
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

    let file_path = if path::Path::new(&arguments[1]).exists() {
        arguments[1].clone().to_string()
    } else {
        return Err("Path does not exist");
    };

    Ok(file_path)
}

fn apply_styles(line: String) -> String {
    let mut styled_content = String::new();

    let mut previous_format_type = FormatType::Normal;

    for character in line.chars() {
        match character {
            c if c.is_digit(10) => {
                styled_content.push_str(&character.to_string().cyan().to_string());
                previous_format_type = FormatType::Number;
            },
            '.' => {
                match previous_format_type {
                    FormatType::Number => {
                        styled_content.push_str(&character.to_string().cyan().to_string());
                    },
                    _ => {
                        styled_content.push_str(&character.to_string());
                    }
                }
            },
            '~' => {
                match previous_format_type {
                    FormatType::Strikethrough => {
                        previous_format_type = FormatType::Normal;
                    },
                    _ => {
                        previous_format_type = FormatType::Strikethrough;
                    },
                }
            },
            '`' => {
                match previous_format_type {
                    FormatType::Highlight => {
                        previous_format_type = FormatType::Normal;
                    },
                    _ => {
                        previous_format_type = FormatType::Highlight;
                    },
                }
            },
            '\'' => {
                match previous_format_type {
                    FormatType::SingleQuoteString => {
                        previous_format_type = FormatType::Normal;
                        styled_content.push_str(&character.to_string().yellow().to_string());
                    },
                    _ => {
                        previous_format_type = FormatType::SingleQuoteString;
                        styled_content.push_str(&character.to_string().yellow().to_string());
                    },
                }
            },
            '"' => {
                match previous_format_type {
                    FormatType::DoubleQuoteString => {
                        previous_format_type = FormatType::Normal;
                        styled_content.push_str(&character.to_string().bright_red().to_string());

                    },
                    _ => {
                        previous_format_type = FormatType::DoubleQuoteString;
                        styled_content.push_str(&character.to_string().bright_red().to_string());
                    },
                }
            },
            '*' => {
                match previous_format_type {
                    FormatType::Italicize => {
                        previous_format_type = FormatType::Normal;
                    },
                    _ => {
                        previous_format_type = FormatType::Italicize;
                    },
                }
            },
            '_' => {
                match previous_format_type {
                    FormatType::Bold => {
                        previous_format_type = FormatType::Normal;
                    },
                    _ => {
                        previous_format_type = FormatType::Bold;
                    },
                }
            },
            _ => {
                match previous_format_type {
                    FormatType::Highlight => {
                        // TODO add a check if terminal supports truecolor
                        styled_content.push_str(&character.to_string().on_truecolor(135, 28, 167).to_string());
                    },
                    FormatType::SingleQuoteString => {
                        styled_content.push_str(&character.to_string().yellow().to_string());
                    },
                    FormatType::DoubleQuoteString => {
                        styled_content.push_str(&character.to_string().bright_red().to_string());
                    },
                    FormatType::Italicize => {
                        styled_content.push_str(&character.to_string().italic().to_string());
                    },
                    FormatType::Bold => {
                        styled_content.push_str(&character.to_string().bold().to_string());
                    },
                    FormatType::Strikethrough => {
                        styled_content.push_str(&character.to_string().strikethrough().to_string());
                    },
                    _ => {
                        // No need for match arm to check FormatType::Number since the
                        // match arm above handles checking if the char is a digit
                        styled_content.push_str(&character.to_string());
                    },
                }
            },
        }
    }

    styled_content.push_str("\n");

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
        process::exit(1);
    }
    println!("File Path: {file_path}"); 

    let file = match fs::File::open(&file_path) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("ERROR: Could not open file {file_path}: {err}");
            process::exit(1)
        },
    };

    let reader = io::BufReader::new(file);

    let mut styled_content = String::new();
    for line in reader.lines() {
        match line {
            Ok(line) => styled_content.push_str(&apply_styles(line)),
            Err(err) => {
                eprintln!("ERROR: Could not read line: {err}");
                process::exit(1)
            }
        }
    }

    println!("{styled_content}");
}
