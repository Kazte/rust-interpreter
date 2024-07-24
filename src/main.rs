use core::panic;
use std::env;
use std::fs;
use std::io::{self, Write};

fn main() {
    // Collect command line arguments
    let args: Vec<String> = env::args().collect();

    // Check if the user has provided the correct number of arguments
    if args.len() < 3 {
        // writeln vs println: writeln! is used to write to a buffer (io::stderr() in this case)
        // println! is used to write to the standard output (stdout)
        writeln!(io::stderr(), "Usage: {} tokenize <filename>", args[0]).unwrap();
        return;
    }

    let command = &args[1];
    let filename = &args[2];

    match command.as_str() {
        "tokenize" => {
            // You can use print statements as follows for debugging, they'll be visible when running tests.
            writeln!(io::stderr(), "Logs from your program will appear here!").unwrap();

            // Read the file contents
            // unwrap_or_else is used to handle the error if the file is not found
            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
                String::new()
            });

            // Check if the file is empty
            if !file_contents.is_empty() {
                scan_token(&file_contents);
            } else {
                // EOF NULL means that the file is empty

                println!("EOF  null"); // Placeholder, remove this line when implementing the scanner
            }
        }
        _ => {
            // Handle unknown commands
            writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
            return;
        }
    }
}

fn scan_token(input: &str) {
    for c in input.chars() {
        match c {
            '(' => println!("LEFT_PAREN ( null"),
            ')' => println!("RIGHT_PAREN ) null"),
            '{' => println!("LEFT_BRACE {{ null"),
            '}' => println!("RIGHT_BRACE }} null"),
            ',' => println!("COMMA , null"),
            '.' => println!("DOT . null"),
            '-' => println!("MINUS - null"),
            '+' => println!("PLUS + null"),
            ';' => println!("SEMICOLON ; null"),
            '*' => println!("STAR * null"),
            _ => panic!("Unexpected character: {}", c),
        }
    }
    println!("EOF  null");
}
