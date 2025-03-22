const OPT_FLAG_HELP: char = 'h';
const FLAG_SEARCH: &str = "-s";
const FLAG_PATH: &str = "-p";

pub enum OptFlag {
    Help,
}

pub struct UserInput<'a> {
    pub search_pattern: &'a str,
    pub file_path:      &'a str,
    pub opt_flags:      Vec<OptFlag>
}

impl<'a> UserInput<'a> {
    fn new_empty() -> Self {
        Self {
            search_pattern: "",
            file_path: "",
            opt_flags: Vec::<OptFlag>::new()
        }
    }
}

pub fn parse_user_input_cli(input: &Vec<String>) -> UserInput {
    let mut user_input_parsed: UserInput = UserInput::new_empty();
    let mut it = input.iter();
    // skip the path to the program
    it.next();

    while let Some(s) = it.next() {
        for c in s.chars() {
            if c == '-' {
                // multiple flags (only optional -- flags that do not require arguments)
                if s.len() > 2 {
                    parse_mult_opt_flags(&s);
                }
                else {
                    // single flag, can only be non-optional flag, with argument
                    match &s[..] {
                        FLAG_PATH => {
                            match it.next() {
                                Some(path_to_file) => {
                                    user_input_parsed.file_path = path_to_file;
                                },
                                None => {
                                    panic!("Argument for the file path is not provided")
                                }
                            }
                        },
                        FLAG_SEARCH => {
                            match it.next() {
                                Some(search_pattern) => {
                                    user_input_parsed.search_pattern = search_pattern;
                                },
                                None => {
                                    panic!("Argument for the file path is not provided")
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

    for c in flags.chars() {
        match c {
            OPT_FLAG_HELP => flags_res.push(OptFlag::Help),
            _ => {
                eprintln!("Unknown flag provided: {}", c);
                eprintln!("Available flags are:\n-h -- help\n-p -- path to the file/dir\n-s -- search pattern");
            }
        }
    }

    return flags_res;
}
