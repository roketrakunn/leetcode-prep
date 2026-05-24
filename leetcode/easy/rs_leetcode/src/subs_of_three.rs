struct Solution;

impl Solution {
    pub fn count_good_substrings(s: String) -> i32 {
        let mut counter = 0 ;
        if s.len() < 3 { 
            return counter;
        }


        for i in 0..s.len() - 2 { 
        if s.as_bytes()[i] != s.as_bytes()[i+1] && 
            s.as_bytes()[i] != s.as_bytes()[i + 2]&&
            s.as_bytes()[i+1] != s.as_bytes()[i+2]
        { 
                counter  += 1;
            } 
        }
        counter
    }
}
