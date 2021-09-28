use std::{env, process, fs};

fn main() {
    let args : Vec<String> = env::args().skip(1).collect();

    let number_of_args = args.len();

    if number_of_args != 1 {
        println!("Expected one argument, given {}", args.len());
        process::exit(1);
    }

    let file_name = &args[0];

    match fs::read_to_string(file_name) {
        Ok(content) => {
            print!("{}", content);
        },
        Err(err) => {
            println!("Could not read file {}", err);
            process::exit(1);
        }
    }
}
