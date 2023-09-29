mod diff;
mod util;

use std::env;
use std::io::Error;
use std::process::exit;

use diff::diff;
use util::get_file_content;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        3 => {
            let file_1 = &args[1];
            let file_2 = &args[2];

            let content_1 = get_file_content(file_1).unwrap_or_else(|err| {
                print_file_err(err, file_1);
                exit(2);
            });
            let content_2 = get_file_content(file_2).unwrap_or_else(|err| {
                print_file_err(err, file_2);
                exit(2);
            });

            let diff = diff(&content_1, &content_2);
            println!("{}", diff);
        }
        4 => {
            let option = &args[1];
            if option.ne("-w") {
                print_usage();
                exit(1);
            }

            let string_1 = &args[2];
            let string_2 = &args[3];

            let diff = diff(&string_1, &string_2);
            println!("{}", diff);
        }
        _ => {
            print_usage();
            exit(1);
        }
    }
}

fn print_usage() {
    println!("usage: diffrs <file_1> <file_2>");
    println!("       diffrs -w <string_1> <string_2>");
}

fn print_file_err(err: Error, file_name: &String) {
    eprintln!("diffrs: {}: {}", file_name, err.to_string());
}
