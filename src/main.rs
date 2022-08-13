use cli_commit_formatter::format_commit_body;
use std::env;
use std::io::stdin;

const DEFAULT_LINE_LENGTH: usize = 72;

fn main() {
    let mut input = String::new();
    println!("Enter commit body to be formatted: ");

    let line_length = parse_line_length_arg();

    match stdin().read_line(&mut input) {
        Ok(_) => {
            let result = format_commit_body(&input, line_length);
            println!("{}", result)
        }
        Err(e) => println!("Something went wrong: {e}"),
    }
}

fn parse_line_length_arg() -> usize {
    let args: Vec<String> = env::args().collect();
    match args.get(1) {
        Some(length) => match length.parse() {
            Ok(val) => val,
            Err(_e) => DEFAULT_LINE_LENGTH,
        },
        None => DEFAULT_LINE_LENGTH,
    }
}
