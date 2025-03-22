use crate::fs_related::FileData;
use crate::str_pattern_match;

pub fn print_help_info() {
    println!("Usage:");
    println!("minigrep [options] -s $pattern -p $filepath");
    print_opt_flags();
}

pub fn print_opt_flags() {
    println!("Options can be:");
    println!("\t-h -- provide information about usage of program");
    println!("\t-r -- do recursive search staring from $filepath");
    println!("\t-i -- ignore case in $pattern and occurences");
}

pub fn print_occurences_in_file(pattern: &String, file_data: &FileData) {
    let occurences = str_pattern_match::find_occurences(&file_data.file_tokens, &pattern);
    println!("{}", file_data.file_path);

    for occurence in occurences.iter() {
        println!("\t{}", occurence);
    }

    println!();
}
