use crate::cli_output::{self, print_opt_flags};

// optional flags
const OPT_FLAG_HELP: u8         = b'h';
const OPT_FLAG_DIR: u8          = b'd';
const OPT_FLAG_RECURSIVE: u8    = b'r';
const OPT_FLAG_IGNORE_CASE: u8  = b'i';
const OPT_FLAG_LINE_NUMBERS: u8 = b'n';
const OPT_FLAG_QUIET: u8        = b'q';

#[derive(PartialEq)]
#[repr(u8)]
pub enum OptFlag {
    Help = OPT_FLAG_HELP,
    Dir = OPT_FLAG_DIR,
    Recursive = OPT_FLAG_RECURSIVE,
    IgnoreCase = OPT_FLAG_IGNORE_CASE,
    LineNumbers = OPT_FLAG_LINE_NUMBERS,
    Quiet = OPT_FLAG_QUIET,
}

// non-optional flags (argument expected)
const FLAG_SEARCH: u8 = b's';
const _FLAG_OUTPUT_TO_FILE: u8 = b'f';
const FLAG_PATH: u8 = b'p';

pub struct UserInput {
    pub search_pattern: String,
    pub search_path:    String,
    pub opt_flags:      Vec<OptFlag>
}

impl UserInput {
    pub fn new_empty() -> Self {
        Self {
            search_pattern: String::new(),
            search_path: String::new(),
            opt_flags: Vec::<OptFlag>::new()
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
            print_opt_flags();
            std::process::exit(1);
        }
    }

    return flags_res;
}

fn match_non_opt_flag(flag: u8, argument: String, user_input: &mut UserInput) {
    match flag {
        FLAG_SEARCH => {
            user_input.search_pattern = argument;
        },
        FLAG_PATH => {
            user_input.search_path = argument;
        }
        _ => unreachable!(),
    }
}

fn match_opt_flag(opt_flag: u8) -> OptFlag {
    match opt_flag {
        OPT_FLAG_HELP => OptFlag::Help,
        OPT_FLAG_DIR => OptFlag::Dir,
        OPT_FLAG_RECURSIVE => OptFlag::Recursive,
        OPT_FLAG_IGNORE_CASE => OptFlag::IgnoreCase,
        OPT_FLAG_LINE_NUMBERS => OptFlag::LineNumbers,
        OPT_FLAG_QUIET => OptFlag::Quiet,
        _ => unreachable!(),
    }
}

fn is_non_opt_flag(flag: u8) -> bool {
    match flag {
        FLAG_SEARCH => true,
        FLAG_PATH => true,
        _ => false,
    }
}

fn is_opt_flag(opt_flag: u8) -> bool {
    match opt_flag {
        OPT_FLAG_HELP => true,
        OPT_FLAG_DIR => true,
        OPT_FLAG_RECURSIVE => true,
        OPT_FLAG_IGNORE_CASE => true,
        OPT_FLAG_LINE_NUMBERS => true,
        OPT_FLAG_QUIET => true,
        _ => false,
    }
}
