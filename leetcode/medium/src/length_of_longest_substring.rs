use std::{char, collections::HashMap, usize};

pub fn length_of_longes_substring(s :String) -> i32 { 

    let mut indices: HashMap<char, usize> = HashMap::new();
    let mut left = 0;
    let mut max_len = 0;

    for (right, ch) in s.chars().enumerate() {
        if let Some(&last_seen) = indices.get(&ch) {
            //
            // TODO: update left (remember the edge case we talked about)
            left = left.max(last_seen + 1);
        }

        // TODO: update indices for ch
        indices.insert(ch, right);

        // TODO: update max_len
        max_len =  max_len.max((right - left + 1) as i32);
    }

    max_len
}

fn main() {
    assert_eq!(length_of_longes_substring("abcabcbb".to_string()), 3);
    assert_eq!(length_of_longes_substring("bbbbb".to_string()), 1);
    assert_eq!(length_of_longes_substring("pwwkew".to_string()), 3);
    assert_eq!(length_of_longes_substring("".to_string()), 0);
    println!("all tests passed");
}
