use std::{env, process, fs, io};
use std::io::Write;

/// Reads content of a file named with path file_path and writes
/// the content to stdout. If an error occurs, the process exits.
fn print_file(file_path: &String) {
    match fs::read(file_path) {
        Ok(content) => {
            match io::stdout().write_all(&content) {
                Err(err) => {
                    println!("Could not write to stdout {}", err);
                    process::exit(1);
                },
                _ => {}
            }

        },
        Err(err) => {
            println!("Could not read file {}", err);
            process::exit(1);
        }
    }
}

fn main() {
    let args : Vec<String> = env::args().collect();
    let number_of_args = args.len();

    match number_of_args {
        // Echo mode. Write std input to std output
        1 => {
            loop {
                let mut str: String = String::new();
                // Use read line to echo for every newline user enters
                match io::stdin().read_line(&mut str) {
                    Ok(n) => {
                        // EOF
                        if n == 0 {
                            process::exit(0);
                        }
                        print!("{}", str);
                    },
                    Err(err) => {
                        println!("Cannot read input {}", err);
                        process::exit(1);
                    }
                }
            }
        },
        // Concatenate all files' contents and print to stdout
        _ => {
            for arg in env::args().skip(1) {
                print_file(&arg);
            }
        }
    }
}
