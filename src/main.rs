use std::{env, process, fs, io};

fn main() {
    let args : Vec<String> = env::args().collect();

    let number_of_args = args.len();

    match number_of_args {
        2 => {
            let file_name = &args[1];

            match fs::read_to_string(file_name) {
                Ok(content) => {
                    print!("{}", content);
                },
                Err(err) => {
                    println!("Could not read file {}", err);
                    process::exit(1);
                }
            }
        },
        1 => {
            loop {
                let mut str: String = String::new();
                match io::stdin().read_line(&mut str) {
                    Ok(_) => {},
                    Err(err) => {
                        println!("Cannot read input {}", err);
                        process::exit(1);
                    }
                }
                print!("{}", str);
            }
        },
        _ => {
            println!("Usage {} [filename]", &args[0]);
            process::exit(1);
        }
    }
}
