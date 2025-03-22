fn match_str(search: &str, pattern: &str) -> bool {
    let search_as_vec: Vec<char> = search.chars().collect();
    let pattern_as_vec: Vec<char> = pattern.chars().collect();
    let search_len = search.len();
    let pattern_len = pattern.len();
    let mut search_ind: usize = 0;
    let mut pattern_ind: usize = 0;
 
    while search_ind < search_len {
        if search_as_vec[search_ind] == pattern_as_vec[pattern_ind] {
            let mut curr_search_ind = search_ind;

            while pattern_ind < pattern_len &&
                search_as_vec[curr_search_ind] == pattern_as_vec[pattern_ind] {
                curr_search_ind += 1;
                pattern_ind += 1;

                // end of 'search' reached
                if curr_search_ind >= search_len {
                    if pattern_ind == pattern_len {
                        return true;
                    }
                    return false;
                }
            }

            // we found a match
            if pattern_ind == pattern_len {
                return true;
            }
            else {
                pattern_ind = 0;
                search_ind += 1;
            }
        }
        else {
            search_ind += 1;
        }
    }

    return false;
}

pub fn find_occurences(tokens: &Vec<String>, pattern: &String) -> Vec<String> {
    let mut occurences = Vec::<String>::with_capacity(tokens.len() / 3);
 
    for token in tokens {
        if match_str(token, pattern) {
            occurences.push(token.clone());
        }
    }

    return occurences;
}

