use std::ops::Sub;

const UP_TO_LOW_CASE_OFFSET_ASCII: u8 = b'a' - b'A';

pub fn find_occurences(tokens: &Vec<String>, pattern: &String, ignore_case: bool) -> Vec<String> {
    let mut occurences = Vec::<String>::with_capacity(tokens.len() / 3);
 
    for token in tokens {
        if match_str(token, pattern, ignore_case) {
            occurences.push(token.clone());
        }
    }

    return occurences;
}

fn match_str(search: &str, pattern: &str, ignore_case: bool) -> bool {
    let search_as_vec: Vec<u8> = search.bytes().collect();
    let pattern_as_vec: Vec<u8> = pattern.bytes().collect();
    return match_vecs::<u8>(&search_as_vec, &pattern_as_vec, ignore_case, UP_TO_LOW_CASE_OFFSET_ASCII)
}

fn match_vecs<T>(search: &Vec<T>, pattern: &Vec<T>, ignore_case: bool, up_to_low_offset: T) -> bool
    where T: Copy + PartialEq + Sub + PartialOrd, <T as Sub>::Output: PartialEq<T>
{
    let search_len = search.len();
    let pattern_len = pattern.len();
    let mut search_ind: usize = 0;
    let mut pattern_ind: usize = 0;

    while search_ind < search_len {
        if cmp_chars::<T>(search[search_ind] as T, pattern[pattern_ind] as T,
            ignore_case, up_to_low_offset) {

            let mut curr_search_ind = search_ind + 1;
            pattern_ind += 1;

            while   curr_search_ind < search_len &&
                    pattern_ind < pattern_len &&
                    cmp_chars::<T>(search[curr_search_ind] as T, pattern[pattern_ind] as T,
                    ignore_case, up_to_low_offset)
            {

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

fn cmp_chars<T>(c1: T, c2: T, ignore_case: bool, up_to_low_offset: T) -> bool
    where T: PartialEq + Sub + PartialOrd, <T as Sub>::Output: PartialEq<T>
{
    if ignore_case {
        if c1 == c2 { return true } else {
            if c1 > c2 {
                return c1 - c2 == up_to_low_offset
            } else {
                return c2 - c1 == up_to_low_offset
            }
        }
    } else {
        return c1 == c2
    }
}
