
struct Solution;


impl Solution {
    pub fn maximum_number_of_string_pairs(words: Vec<String>) -> i32 {
        let mut count :i32 = 0;

        for i in 0..words.len() { 
            let w1 = &words[i]; 
            for j in i+1..words.len() { 
                let w2 : String = words[j].chars().rev().collect();
                if *w1 == w2{ 
                    count += 1;
                }
            }
        }
        count
    }
}
