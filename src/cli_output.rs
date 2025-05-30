use std::fs::File;
use std::io::{ErrorKind, Write};

use crate::cli_input::{OptFlag, UserInput};
use crate::fs_related::{FileData, Token};
use crate::str_pattern_match;

pub fn print_help_info() {
    println!("General Usage:");
    println!("minigrep [options] -s $pattern -p $filepath [-f $output_to_file_path] [-e .git,.png,.exe]");
    print_opt_flags();
    print_arg_flags();
}

pub fn print_opt_flags() {
    println!("Options can be:");
    println!("\t-h -- provide information about usage of program");
    println!("\t-q -- make program quiet, error logs would not be displayed, recommended");
    println!("\t-d -- search a directory starting from $filepath (by default program expect a file)");
    println!("\t-r -- do recursive search starting from $filepath");
    println!("\t-i -- ignore case in $pattern and occurences");
    println!("\t-n -- output line numbers");
}

pub fn print_arg_flags() {
    println!("You can also include options that accept an argument:");
    println!("\t-f -- write all program output to the file, instead of stdin:\n-f $output_to_file_path");
    println!("\t-e -- exclude searching from paths which contain patterns:\n-e .git,.png,.exe");
}


pub fn print_occurences_in_file(pattern: &String, file_data: FileData, user_input: &UserInput) {
    let ignore_case = user_input.has_opt_flag(OptFlag::IgnoreCase);

    let occurences = str_pattern_match::find_occurences(file_data.file_tokens, &pattern, ignore_case);
    if occurences.len() == 0 {
        return;
    }

    println!("{}", file_data.file_path);

    for occurence in occurences.iter() {
        match occurence {
            Token::TokenStr(ref token_str) => {
                println!("\t{}", token_str);
            },
            Token::TokenStrLine(ref token_line) => {
                println!("\t{}. {}", token_line.line_num, token_line.contents);
            }
        }
    }
}

pub fn write_occurences_to_output_file(pattern: &String, file_data: FileData, output_file_path: &String, user_input: &UserInput) {
    let mut output_file = match File::options().append(true).create(true).open(output_file_path) {
        Ok(opened_file) => opened_file,
        Err(err) => match err.kind() {
            ErrorKind::PermissionDenied => {
                eprintln!("File for writing output to doesn't have a permission to access\nprovided path: {}", output_file_path);
                return ();
            },
            ErrorKind::IsADirectory => {
                eprintln!("File for writing output to is a directory\nprovided path: {}", output_file_path);
                return ();
            },
            _ => {
                eprintln!("Unknown error occurred when attempting to open the file for writing program output");
                eprintln!("Path was: {}", output_file_path);
                return ();
            }
        }
    };

    let ignore_case = user_input.has_opt_flag(OptFlag::IgnoreCase);

    let occurences = str_pattern_match::find_occurences(file_data.file_tokens, &pattern, ignore_case);
    if occurences.len() == 0 {
        return;
    }

    let _ = output_file.write(file_data.file_path.as_bytes()).expect("Writing to the file failed");
    let _ = output_file.write(b"\n");

    for occurence in occurences.iter() {
        match occurence {
            Token::TokenStr(ref token_str) => {
                let _ = output_file.write(b"\t");
                let _ = output_file.write(token_str.as_bytes()).expect("Writing to the file failed");
                let _ = output_file.write(b"\n");
            },
            Token::TokenStrLine(ref token_line) => {
                let _ = output_file.write(b"\t");
                let line_n_to_print: Vec<u8> = token_line.line_num.to_string().bytes().collect();
                let _ = output_file.write(&line_n_to_print[..]);
                let _ = output_file.write(b". ");
                let _ = output_file.write(token_line.contents.as_bytes()).expect("Writing to the file failed");
                let _ = output_file.write(b"\n");
            }
        }
    }
}
