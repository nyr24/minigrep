use std::fs::{File, FileType};
use std::io::{ErrorKind, Read};
use std::str::FromStr;
use crate::cli_input::{UserInput, OptFlag};

pub struct FileData {
    pub file_path:          String,
    pub file_tokens:        Vec<String>,
}

pub fn do_search(user_input: &UserInput) -> Vec<FileData> {
    let do_recursive_search = user_input.has_opt_flag(OptFlag::Recursive);
    let do_dir_search = do_recursive_search || user_input.has_opt_flag(OptFlag::Dir);
    let be_quiet = user_input.has_opt_flag(OptFlag::Quiet);

    let mut file_search_data: Vec<FileData>;

    if do_dir_search {
        file_search_data = Vec::with_capacity(10);
        search_dir(&user_input.search_path, &mut file_search_data, do_recursive_search, be_quiet);
    } else {
        file_search_data = Vec::with_capacity(1);
        let file_data = get_file_data(&user_input.search_path, be_quiet);
        file_search_data.push(file_data);
    }

    return file_search_data;
}

fn search_dir(search_path: &str, file_data_out: &mut Vec<FileData>, recursive: bool, quiet: bool) {
    let dir_iter = match std::fs::read_dir(search_path) {
        Ok(it) => it,
        Err(err) => match err.kind() {
            ErrorKind::PermissionDenied => {
                if !quiet {
                    eprintln!("Permission denied to access dir by path: {}", search_path);
                }
                std::process::exit(1);
            },
            ErrorKind::NotADirectory => {
                if !quiet {
                    eprintln!("Found a file, not a directory with provided path: {}", search_path);
                    eprintln!("Consider unspecify flags -d,-r if presented");
                }
                std::process::exit(1);
            },
            ErrorKind::NotFound => {
                if !quiet {
                    eprintln!("Directory wasn't found by path: {}", search_path);
                }
                std::process::exit(1);
            },
            _ => {
                if !quiet {
                    eprintln!("Unexpected error occured when opening directory")
                }
                std::process::exit(1)
            }
        }
    };

    for dir_entry_opt in dir_iter {
        match dir_entry_opt {
            Ok(dir_entry) => {
                if let Ok(file_type) = dir_entry.file_type() {
                    let entry_full_path = match dir_entry.path().to_str() {
                        Some(s) => {
                            String::from_str(s).unwrap()
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
                        let file_tokens = get_file_tokens(&entry_full_path, quiet);

                        let file_data = FileData {
                            file_path: entry_full_path,
                            file_tokens,
                        };

                        file_data_out.push(file_data);
                    }
                    // if entry is dir
                    else if recursive && FileType::is_dir(&file_type) {
                        search_dir(&entry_full_path, file_data_out, recursive, quiet);
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

fn get_file_data(file_path: &str, quiet: bool) -> FileData {
    let mut file = match File::open(file_path) {
        Ok(f) => f,
        Err(err) => match err.kind() {
            ErrorKind::PermissionDenied => {
                if !quiet {
                    eprintln!("Permission denied for file access at path: {}", file_path);
                }
                std::process::exit(1);
            },
            ErrorKind::NotFound => {
                if !quiet {
                    eprintln!("File wasn't found at path: {}", file_path);
                }
                std::process::exit(1);
            },
            ErrorKind::IsADirectory => {
                if !quiet {
                    eprintln!("Directory found at path, not a file: {}", file_path);
                    eprintln!("Consider specify flags for directory search: -d,-r");
                }
                std::process::exit(1);
            },
            _ => {
                eprintln!("Unexpected error");
                std::process::exit(1);
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
                std::process::exit(1);
            },
            ErrorKind::NotFound => {
                if !quiet {
                    eprintln!("File wasn't found at path: {}", file_path);
                }
                std::process::exit(1);
            },
            ErrorKind::IsADirectory => {
                if !quiet {
                    eprintln!("Directory found at path, not a file: {}", file_path);
                }
                std::process::exit(1);
            },
            _ => {
                eprintln!("Unexpected error");
                std::process::exit(1);
            }
        }
    }

    let file_tokens = parse_to_tokens(&contents_buff);
    let file_path = String::from_str(file_path).unwrap();

    return FileData {
        file_path,
        file_tokens,
    }
}

fn get_file_tokens(file_path: &str, quiet: bool) -> Vec<String> {
    let mut file = match File::open(file_path) {
        Ok(file) => file,
        Err(err) => match err.kind() {
            ErrorKind::PermissionDenied => {
                if !quiet {
                    eprintln!("Permission denied for file access at path: {}", file_path);
                }
                std::process::exit(1);
            },
            ErrorKind::NotFound => {
                if !quiet {
                    eprintln!("File wasn't found at path: {}", file_path);
                }
                std::process::exit(1);
            },
            ErrorKind::IsADirectory => {
                if !quiet {
                    eprintln!("Directory found at path, not a file: {}", file_path);
                }
                std::process::exit(1);
            },
            _ => {
                eprintln!("Unexpected error");
                std::process::exit(1);
            }
        }
    };

    let mut contents_buff = String::new();
    File::read_to_string(&mut file, &mut contents_buff).expect("File was not readed to the end");
    let file_tokens = parse_to_tokens(&contents_buff);

    return file_tokens;
}

fn parse_to_tokens(file_contents: &String) -> Vec<String> {
    let mut tokens = Vec::<String>::with_capacity(file_contents.len() / 5);

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
            tokens.push(String::from_str(curr_slice).unwrap());
        }
    }

    return tokens;
}
