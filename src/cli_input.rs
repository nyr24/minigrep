use crate::cli_output;
use crate::fs_related::char_slice_to_str;

// optional flags
const OPT_FLAG_HELP: u8             = b'h';
const OPT_FLAG_DIR: u8              = b'd';
const OPT_FLAG_RECURSIVE: u8        = b'r';
const OPT_FLAG_LINE_NUMBERS: u8     = b'n';
const OPT_FLAG_QUIET: u8            = b'q';
const OPT_FLAG_IGNORE_CASE: u8      = b'i';

#[derive(PartialEq)]
#[repr(u8)]
pub enum OptFlag {
    Help = OPT_FLAG_HELP,
    Dir = OPT_FLAG_DIR,
    Recursive = OPT_FLAG_RECURSIVE,
    LineNumbers = OPT_FLAG_LINE_NUMBERS,
    Quiet = OPT_FLAG_QUIET,
    IgnoreCase = OPT_FLAG_IGNORE_CASE,
}

// non-optional flags (argument expected)
const FLAG_SEARCH: u8 = b's';
const FLAG_OUTPUT_TO_FILE: u8 = b'f';
const FLAG_PATH: u8 = b'p';
const FLAG_EXCLUDE_PATHS: u8 = b'e';

pub struct UserInput {
    pub search_pattern:     String,
    pub search_path:        String,
    pub output_file_path:   Option<String>,
    pub exclude_paths:      Option<Vec<String>>,
    pub opt_flags:          Vec<OptFlag>
}

impl UserInput {
    pub fn new_empty() -> Self {
        Self {
            search_pattern:     String::new(),
            search_path:        String::new(),
            output_file_path:   None,
            exclude_paths:      None,
            opt_flags:          Vec::<OptFlag>::new()
        }
    }

    pub fn has_opt_flag(&self, flag: OptFlag) -> bool {
        return self.opt_flags.contains(&flag);
    }
}

pub fn parse_user_input_cli(input: Vec<String>) -> UserInput {
    let mut user_input_parsed: UserInput = UserInput::new_empty();
    let mut it = input.into_iter();
    // skip the path to the program
    it.next();

    // are we currently parsing a flag or not
    let mut is_on_flag;

    'outer: while let Some(curr_cli_arg) = it.next() {
        is_on_flag = false;

        'inner: for c in curr_cli_arg.bytes() {
            if c == b'-' {
                // multiple flags (only optional flags that do not require arguments)
                if curr_cli_arg.len() > 2 {
                    user_input_parsed.opt_flags = parse_mult_opt_flags(&curr_cli_arg[1..]);
                    continue 'outer;
                }
                else {
                    is_on_flag = true;
                    continue 'inner;
                }
            }
            else {
                if is_opt_flag(c) {
                    user_input_parsed.opt_flags.push(match_opt_flag(c));
                }
                else if is_non_opt_flag(c) {
                    match it.next() {
                        Some(next_cli_arg) => {
                            match_non_opt_flag(c, next_cli_arg, &mut user_input_parsed);
                        },
                        None => {
                            eprintln!("Argument for non-optional flag {} is missing", c as char);
                            cli_output::print_help_info();
                            std::process::exit(1);
                        }
                    }
                }
                else {
                    if is_on_flag {
                        eprintln!("Unknown flag provided: {}", c as char);
                        cli_output::print_opt_flags();
                        std::process::exit(1);
                    } else {
                        eprintln!("Unknown input provided: {}", &curr_cli_arg);
                        cli_output::print_help_info();
                        std::process::exit(1);
                    }
                }
            }
        }
    }

    return user_input_parsed;
}

fn parse_mult_opt_flags(flags: &str) -> Vec<OptFlag> {
    let mut flags_res = Vec::<OptFlag>::new();

    for c in flags.bytes() {
        if is_opt_flag(c) {
            flags_res.push(match_opt_flag(c));
        }
        else {
            eprintln!("Unknown optional flag provided: {}", c as char);
            cli_output::print_opt_flags();
            std::process::exit(1);
        }
    }

    return flags_res;
}

fn match_non_opt_flag(flag: u8, argument: String, user_input: &mut UserInput) {
    match flag {
        FLAG_SEARCH => user_input.search_pattern = argument,
        FLAG_PATH => user_input.search_path = argument,
        FLAG_OUTPUT_TO_FILE => user_input.output_file_path = Some(argument),
        FLAG_EXCLUDE_PATHS => user_input.exclude_paths = parse_exclude_paths(&argument, ','),
        _ => unreachable!(),
    }
}

fn match_opt_flag(opt_flag: u8) -> OptFlag {
    match opt_flag {
        OPT_FLAG_HELP => OptFlag::Help,
        OPT_FLAG_DIR => OptFlag::Dir,
        OPT_FLAG_RECURSIVE => OptFlag::Recursive,
        OPT_FLAG_LINE_NUMBERS => OptFlag::LineNumbers,
        OPT_FLAG_QUIET => OptFlag::Quiet,
        OPT_FLAG_IGNORE_CASE => OptFlag::IgnoreCase,
        _ => unreachable!(),
    }
}

fn is_non_opt_flag(flag: u8) -> bool {
    match flag {
        FLAG_SEARCH => true,
        FLAG_PATH => true,
        FLAG_OUTPUT_TO_FILE => true,
        FLAG_EXCLUDE_PATHS => true,
        _ => false,
    }
}

fn is_opt_flag(opt_flag: u8) -> bool {
    match opt_flag {
        OPT_FLAG_HELP => true,
        OPT_FLAG_DIR => true,
        OPT_FLAG_RECURSIVE => true,
        OPT_FLAG_LINE_NUMBERS => true,
        OPT_FLAG_QUIET => true,
        OPT_FLAG_IGNORE_CASE => true,
        _ => false,
    }
}

fn parse_exclude_paths(exclude_paths: &String, splitter: char) -> Option<Vec<String>> {
    let exclude_paths = split_str_into_vec(&exclude_paths, splitter);
    if exclude_paths.len() == 0 {
        return None;
    } else {
        return Some(exclude_paths);
    }
}

fn split_str_into_vec(s: &String, splitter: char) -> Vec<String> {
    let mut result = Vec::<String>::with_capacity(s.len() / 5);
    let char_vec: Vec<char> = s.chars().collect();
    let char_vec_last_ind = char_vec.len() - 1;
    let mut curr_slice_start: usize = 0;

    for (ind, ch) in char_vec.iter().enumerate() {
        if *ch == splitter && ((ind - curr_slice_start) > 0) {
            let splitted_part = char_slice_to_str(&char_vec[curr_slice_start..ind]);
            result.push(splitted_part);
            curr_slice_start = ind + 1;
        }
        else if ind == char_vec_last_ind && (ind - curr_slice_start > 0) {
            let splitted_part = char_slice_to_str(&char_vec[curr_slice_start..ind]);
            result.push(splitted_part);
        }
    }

    return result;
}
