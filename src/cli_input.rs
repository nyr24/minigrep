use crate::cli_output;

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

const FLAG_SEARCH: &str = "-s";
const FLAG_PATH: &str = "-p";
const FLAG_OUTPUT_TO_FILE: &str = "-f";

pub struct UserInput {
    pub search_pattern: String,
    pub path:           String,
    pub opt_flags:      Vec<OptFlag>
}

impl UserInput {
    pub fn new_empty() -> Self {
        Self {
            search_pattern: String::new(),
            path: String::new(),
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

    while let Some(s) = it.next() {
        for c in s.chars() {
            if c == '-' {
                // multiple flags (only optional -- flags that do not require arguments)
                if s.len() > 2 {
                    user_input_parsed.opt_flags = parse_mult_opt_flags(&s);
                }
                else {
                    // single flag, can only be non-optional flag, with argument
                    match &s[..] {
                        FLAG_PATH => {
                            match it.next() {
                                Some(path) => {
                                    user_input_parsed.path = path;
                                },
                                None => {
                                    eprintln!("Argument for the file path is not provided");
                                    cli_output::print_help_info();
                                    std::process::exit(0);
                                }
                            }
                        },
                        FLAG_SEARCH => {
                            match it.next() {
                                Some(search_pattern) => {
                                    user_input_parsed.search_pattern = search_pattern;
                                },
                                None => {
                                    eprintln!("Argument for the search pattern is not provided");
                                    cli_output::print_help_info();
                                    std::process::exit(0);
                                }
                            }
                        },
                        _ => ()
                    }
                }
            }
        }
    }

    return user_input_parsed;
}

fn parse_mult_opt_flags(flags: &String) -> Vec<OptFlag> {
    let mut flags_res = Vec::<OptFlag>::new();

    for c in flags.bytes() {
        match c {
            OPT_FLAG_HELP => flags_res.push(OptFlag::Help),
            OPT_FLAG_DIR => flags_res.push(OptFlag::Dir),
            OPT_FLAG_RECURSIVE => flags_res.push(OptFlag::Recursive),
            OPT_FLAG_IGNORE_CASE => flags_res.push(OptFlag::IgnoreCase),
            OPT_FLAG_LINE_NUMBERS => flags_res.push(OptFlag::LineNumbers),
            OPT_FLAG_QUIET => flags_res.push(OptFlag::Quiet),
            _ => {
                eprintln!("Unknown flag provided: {}", c);
                cli_output::print_opt_flags();
            }
        }
    }

    return flags_res;
}
