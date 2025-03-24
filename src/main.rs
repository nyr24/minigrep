use self::cli_input::OptFlag;
use self::cli_output::{print_occurences_in_file, write_occurences_to_output_file};
use self::fs_related::do_search;

mod cli_input;
mod cli_output;
mod str_pattern_match;
mod fs_related;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        cli_output::print_help_info();
        std::process::exit(0);
    }

    let user_input = cli_input::parse_user_input_cli(args);
    if user_input.has_opt_flag(OptFlag::Help) {
        cli_output::print_help_info();
        std::process::exit(0);
    }

    let file_data = do_search(&user_input);

    match &user_input.output_file_path {
        Some(output_file_path) => {
            for file_d in file_data.iter() {
                write_occurences_to_output_file(&user_input.search_pattern, file_d, output_file_path, &user_input);
            }
        },
        None => {
            for file_d in file_data.iter() {
                print_occurences_in_file(&user_input.search_pattern, file_d, &user_input);
            }
        }
    }

}
