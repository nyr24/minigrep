use std::fs::File;
use std::io::{ErrorKind, Read};

struct UserInput<'a> {
    query:      &'a String,
    file_path:  &'a String,
}

fn read_file(file_path: &str) -> String {
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

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        panic!("You should provide at least 2 arguments for the program:\n1) String to search for\n2) Path to the file");
    }

    let user_input = UserInput {
        query: &args[1],
        file_path: &args[2]
    };

    let contents = read_file(&user_input.file_path);

    println!("query: {}", user_input.query);
    println!("path: {}", user_input.file_path);
    print!("contents are:\n{}", contents);
}
