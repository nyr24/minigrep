use std::ops::Sub;

const UP_TO_LOW_CASE_OFFSET_ASCII: u8 = b'a' - b'A';
const UP_TO_LOW_CASE_OFFSET_CYRILLIC: u16 = 'а' as u16 - 'А' as u16;

pub fn find_occurences(tokens: &Vec<String>, pattern: &String, ignore_case: bool, cyrillic_mode: bool) -> Vec<String> {
    let mut occurences = Vec::<String>::with_capacity(tokens.len() / 3);
 
    for token in tokens {
        if match_str(token, pattern, ignore_case, cyrillic_mode) {
            occurences.push(token.clone());
        }
    }

    return occurences;
}

fn match_str(search: &str, pattern: &str, ignore_case: bool, cyrillic_mode: bool) -> bool {
    if !cyrillic_mode {
        return match_str_ascii(search, pattern, ignore_case)
    } else {
        return match_str_cyrillic(search, pattern, ignore_case)
    }
}

fn match_str_ascii(search: &str, pattern: &str, ignore_case: bool) -> bool {
    let search_as_vec: Vec<u8> = search.bytes().collect();
    let pattern_as_vec: Vec<u8> = pattern.bytes().collect();
    let search_len = search_as_vec.len();
    let pattern_len = pattern_as_vec.len();
    let mut search_ind: usize = 0;
    let mut pattern_ind: usize = 0;

    while search_ind < search_len {
        if cmp_chars(search_as_vec[search_ind], pattern_as_vec[pattern_ind], ignore_case,
            UP_TO_LOW_CASE_OFFSET_ASCII) {

            let mut curr_search_ind = search_ind + 1;
            pattern_ind += 1;

            while   curr_search_ind < search_len &&
                    pattern_ind < pattern_len &&
                    cmp_chars(search_as_vec[curr_search_ind], pattern_as_vec[pattern_ind], ignore_case,
                    UP_TO_LOW_CASE_OFFSET_ASCII)
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

fn match_str_cyrillic(search: &str, pattern: &str, ignore_case: bool) -> bool {
    let search_as_vec: Vec<u8> = search.bytes().collect();
    let pattern_as_vec: Vec<u8> = pattern.bytes().collect();

    let search_as_vec = transform_u8_vec_to_u16_vec(&search_as_vec);
    let pattern_as_vec = transform_u8_vec_to_u16_vec(&pattern_as_vec);

    let search_len = search_as_vec.len();
    let pattern_len = pattern_as_vec.len();
    let mut search_ind: usize = 0;
    let mut pattern_ind: usize = 0;

    while search_ind < search_len {
        if cmp_chars(search_as_vec[search_ind], pattern_as_vec[pattern_ind], ignore_case,
            UP_TO_LOW_CASE_OFFSET_CYRILLIC) {

            let mut curr_search_ind = search_ind + 1;
            pattern_ind += 1;

            while   curr_search_ind < search_len &&
                    pattern_ind < pattern_len &&
                    cmp_chars(search_as_vec[curr_search_ind], pattern_as_vec[pattern_ind], ignore_case,
                    UP_TO_LOW_CASE_OFFSET_CYRILLIC)
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

fn transform_u8_vec_to_u16_vec(input: &Vec<u8>) -> Vec<u16> {
    let len = input.len();
    let is_odd_len = len & 1 == 1;
    let capacity = if is_odd_len { (len + 1) / 2 } else { len / 2 };
    let mut output = Vec::<u16>::with_capacity(capacity);
    let mut curr_output_u16: u16 = 0;
    let mut curr_byte_tuple: (u8, u8) = (0, 0);

    for (i, byte) in input.iter().enumerate() {
        if i & 1 == 0 {
            curr_byte_tuple.0 = *byte;
        } else {
            curr_byte_tuple.1 = *byte;
            curr_output_u16 |= curr_byte_tuple.0 as u16;
            curr_output_u16 |= curr_byte_tuple.1 as u16;
 
            output.push(curr_output_u16);

            curr_output_u16 = 0;
            curr_byte_tuple.0 = 0;
            curr_byte_tuple.1 = 0;
        }
    }

    return output;
}
