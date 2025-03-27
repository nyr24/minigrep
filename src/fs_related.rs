use std::fs::{File, FileType};
use std::io::{ErrorKind, Read};
use std::str::FromStr;
use crate::cli_input::{UserInput, OptFlag};

pub struct TokenWithLine {
    pub contents: String,
    pub line_num: usize,
}

pub enum Token {
    TokenStr(String),
    TokenStrLine(TokenWithLine),
}

pub struct FileData {
    pub file_path:          String,
    pub file_tokens:        Vec<Token>,
}

pub fn do_search(user_input: &UserInput) -> Vec<FileData> {
    let do_recursive_search = user_input.has_opt_flag(OptFlag::Recursive);
    let do_dir_search = do_recursive_search || user_input.has_opt_flag(OptFlag::Dir);
    let be_quiet = user_input.has_opt_flag(OptFlag::Quiet);
    let line_numbers = user_input.has_opt_flag(OptFlag::LineNumbers);

    let mut file_search_data: Vec<FileData>;

    if do_dir_search {
        file_search_data = Vec::with_capacity(10);
        match &user_input.exclude_paths {
            Some(excl_paths) => {
                if filter_path(&user_input.search_path, excl_paths) {
                    search_dir(&user_input.search_path, &mut file_search_data, do_recursive_search, be_quiet, line_numbers, user_input.exclude_paths.as_ref());
                }
            },
            None => {
                search_dir(&user_input.search_path, &mut file_search_data, do_recursive_search, be_quiet, line_numbers, None);
            }
        }
    } else {
        file_search_data = Vec::with_capacity(1);
        match &user_input.exclude_paths {
            Some(excl_paths) => {
                if filter_path(&user_input.search_path, excl_paths) {
                    if let Some(file_data) = get_file_data(&user_input.search_path, be_quiet, line_numbers) {
                        file_search_data.push(file_data);
                    }
                }
            },
            None => {
                if let Some(file_data) = get_file_data(&user_input.search_path, be_quiet, line_numbers) {
                    file_search_data.push(file_data);
                }
            }
        }
    }

    return file_search_data;
}

fn search_dir(search_path: &str, file_data_out: &mut Vec<FileData>, recursive: bool, quiet: bool, line_numbers: bool, exclude_paths: Option<&Vec<String>>) {
    let dir_iter = match std::fs::read_dir(search_path) {
        Ok(it) => it,
        Err(err) => match err.kind() {
            ErrorKind::PermissionDenied => {
                if !quiet {
                    eprintln!("Permission denied to access dir by path: {}", search_path);
                }
                return ();
            },
            ErrorKind::NotADirectory => {
                if !quiet {
                    eprintln!("Found a file, not a directory with provided path: {}", search_path);
                    eprintln!("Consider unspecify flags -d,-r if presented");
                }
                return ();
            },
            ErrorKind::NotFound => {
                if !quiet {
                    eprintln!("Directory wasn't found by path: {}", search_path);
                }
                return ();
            },
            _ => {
                if !quiet {
                    eprintln!("Unexpected error occured when opening directory")
                }
                return ();
            }
        }
    };

    for dir_entry_opt in dir_iter {
        match dir_entry_opt {
            Ok(dir_entry) => {
                if let Ok(file_type) = dir_entry.file_type() {
                    let entry_full_path = match dir_entry.path().to_str() {
                        Some(slice) => {
                            match String::from_str(slice) {
                                Ok(s) => s,
                                Err(_) => continue,
                            }
                        },
                        None => {
                            if !quiet {
                                eprintln!("Path is not a valid unicode, skipping");
                            }
                            continue;
                        }
                    };

                    // if entry is file
                    if FileType::is_file(&file_type) {
                        match exclude_paths {
                            Some(excl_paths) => {
                                if filter_path(&entry_full_path, excl_paths) {
                                    if let Some(tokens) = get_file_tokens(&entry_full_path, quiet, line_numbers) {
                                        let file_data = FileData {
                                            file_path: entry_full_path,
                                            file_tokens: tokens,
                                        };
                                        file_data_out.push(file_data);
                                    };
                                }
                            },
                            None => {
                                if let Some(tokens) = get_file_tokens(&entry_full_path, quiet, line_numbers) {
                                    let file_data = FileData {
                                        file_path: entry_full_path,
                                        file_tokens: tokens,
                                    };
                                    file_data_out.push(file_data);
                                };
                            }
                        }
                    }
                    // if entry is dir
                    else if recursive && FileType::is_dir(&file_type) {
                        match exclude_paths {
                            Some(excl_paths) => {
                                if filter_path(&entry_full_path, excl_paths) {
                                    search_dir(&entry_full_path, file_data_out, recursive, quiet, line_numbers, exclude_paths);
                                }
                            },
                            None => {
                                search_dir(&entry_full_path, file_data_out, recursive, quiet, line_numbers, None);
                            }
                        }
                    }
                } else {
                    if !quiet {
                        eprintln!("Can't get a file type for the path: {:?}\nskipping", dir_entry.path());
                    }
                }
            },
            Err(err) => {
                if !quiet {
                    eprintln!("Dir entry data error: {}\nskipping", err);
                }
            },
        }
    }
}

fn get_file_data(file_path: &String, quiet: bool, line_numbers: bool) -> Option<FileData> {
    let mut file = match File::open(file_path) {
        Ok(f) => f,
        Err(err) => match err.kind() {
            ErrorKind::PermissionDenied => {
                if !quiet {
                    eprintln!("Permission denied for file access at path: {}", file_path);
                }
                return None
            },
            ErrorKind::NotFound => {
                if !quiet {
                    eprintln!("File wasn't found at path: {}", file_path);
                }
                return None
            },
            ErrorKind::IsADirectory => {
                if !quiet {
                    eprintln!("Directory found at path, not a file: {}", file_path);
                    eprintln!("Consider specify flags for directory search: -d,-r");
                }
                return None
            },
            _ => {
                if !quiet {
                    eprintln!("Unexpected error when opening a file: {}", file_path);
                }
                return None
            }
        }
    };

    let mut contents_buff = String::new();

    match file.read_to_string(&mut contents_buff) {
        Ok(_) => (),
        Err(err) => match err.kind() {
            ErrorKind::PermissionDenied => {
                if !quiet {
                    eprintln!("Permission denied for file access at path: {}", file_path);
                }
                return None
            },
            ErrorKind::NotFound => {
                if !quiet {
                    eprintln!("File wasn't found at path: {}", file_path);
                }
                return None
            },
            ErrorKind::IsADirectory => {
                if !quiet {
                    eprintln!("Directory found at path, not a file: {}", file_path);
                }
                return None
            },
            _ => {
                if !quiet {
                    eprintln!("Unexpected error for file access at path: {}", file_path);
                }
                return None
            }
        }
    }

    let file_tokens = parse_to_tokens(&contents_buff, line_numbers);
    let file_path = String::from_str(file_path).unwrap();

    return Some(FileData {
        file_path,
        file_tokens,
    });
}

fn get_file_tokens(file_path: &str, quiet: bool, line_numbers: bool) -> Option<Vec<Token>> {
    let mut file = match File::open(file_path) {
        Ok(file) => file,
        Err(err) => match err.kind() {
            ErrorKind::PermissionDenied => {
                if !quiet {
                    eprintln!("Permission denied for file access at path: {}", file_path);
                }
                return None;
            },
            ErrorKind::NotFound => {
                if !quiet {
                    eprintln!("File wasn't found at path: {}", file_path);
                }
                return None;
            },
            ErrorKind::IsADirectory => {
                if !quiet {
                    eprintln!("Directory found at path, not a file: {}", file_path);
                }
                return None;
            },
            _ => {
                eprintln!("Unexpected error for file access at path: {}", file_path);
                return None;
            }
        }
    };

    let mut contents_buff = String::new();

    match File::read_to_string(&mut file, &mut contents_buff) {
        Ok(_) => {
            let file_tokens = parse_to_tokens(&contents_buff, line_numbers);
            return Some(file_tokens);
        },
        Err(err) => {
            if !quiet {
                eprintln!("Error while reading a file from path: {}", file_path);
                eprintln!("Error: {}", err);
            }
            return None
        }
    }
}

fn parse_to_tokens(file_contents: &String, line_numbers: bool) -> Vec<Token> {
    let mut tokens = Vec::<Token>::with_capacity(file_contents.len() / 5);

    let it: Vec<char> = file_contents.chars().collect();
    let len = it.len();
    let mut ind: usize = 0;
    let mut curr_slice: &[char];
    let mut curr_slice_start: usize;
    let mut curr_line_num: usize = 0;

    while ind < len {
        // skipping whitespace
        while ind < len && char::is_whitespace(it[ind]) {
            if line_numbers && it[ind] == '\n' {
                curr_line_num += 1;
            }
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
            curr_slice = &it[curr_slice_start..ind];
            if line_numbers {
                let new_token = TokenWithLine {
                    contents: char_slice_to_str(curr_slice),
                    line_num: curr_line_num,
                };
                tokens.push(Token::TokenStrLine(new_token));
            } else {
                tokens.push(Token::TokenStr(char_slice_to_str(curr_slice)));
            }
        }
    }

    return tokens;
}

pub fn char_slice_to_str(char_slice: &[char]) -> String {
    let mut s = String::with_capacity(char_slice.len());

    for c in char_slice {
        s.push(*c);
    }

    return s;
}

fn filter_path(path: &String, exclude_paths: &Vec<String>) -> bool {
    for excl_path in exclude_paths.iter() {
        if path.contains(excl_path) {
            return false;
        }
    }

    return true;
}
