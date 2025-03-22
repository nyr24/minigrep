use std::fs::File;
use std::io::{ErrorKind, Read};

mod cli;
mod str_match;

use self::str_match::match_str;

fn get_file_contents(file_path: &str) -> String {
    let mut file = match File::open(file_path) {
        Ok(f) => f,
        Err(err) => match err.kind() {
            ErrorKind::PermissionDenied => {
                eprintln!("Permission denied for file access at path: {}", file_path);
                panic!("Permission error");
            },
            ErrorKind::NotFound => {
                eprintln!("File wasn't found at path: {}", file_path);
                panic!("File missing error");
            },
            _ => panic!("Unexpected error")
        }
    };

    let mut contents_buff = String::new();
    File::read_to_string(&mut file, &mut contents_buff).expect("file was not readed to the end");
    return contents_buff;
}

fn parse_to_tokens(file_contents: &String) -> Vec<&str> {
    let mut tokens = Vec::<&str>::with_capacity(file_contents.len() / 5);

    let it: Vec<char> = file_contents.chars().collect();
    let len = it.len();
    let mut ind: usize = 0;
    let mut curr_slice: &str;
    let mut curr_slice_start: usize;

    while ind < len {
        // skipping whitespace
        while ind < len && char::is_whitespace(it[ind]) {
            ind += 1;
        }

        if ind >= len {
            break;
        }

        curr_slice_start = ind;

        while ind < len && !char::is_whitespace(it[ind]) {
            ind += 1;
        }

        if ind - curr_slice_start != 0 {
            curr_slice = &file_contents[curr_slice_start..ind];
            tokens.push(curr_slice);
        }
    }

    return tokens;
}

fn find_occurences<'a>(tokens: &'a Vec<&'a str>, pattern: &str) -> Vec<&'a str> {
    let mut occurences = Vec::<&str>::with_capacity(tokens.len() / 3);
 
    for token in tokens {
        if match_str(*token, pattern) {
            occurences.push(*token);
        }
    }

    return occurences;
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        panic!("You should provide at least 2 arguments for the program:\n1) String to search for\n2) Path to the file");
    }

    let user_input = cli::parse_user_input_cli(&args);

    let contents = get_file_contents(&user_input.file_path);
    let tokens = parse_to_tokens(&contents);
    let occurences = find_occurences(&tokens, &user_input.search_pattern);

    for occur in occurences.iter() {
        println!("{}", occur);
    }
}
