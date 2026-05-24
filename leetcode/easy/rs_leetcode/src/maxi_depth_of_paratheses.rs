
struct Solution; 
impl Solution {

    pub fn max_depth(s: String) -> i32 {
        let mut  counter = 0;
        let mut maxie : i32 = 0; 

        for c in s.chars() { 
            
            if c == '(' { 
                counter += 1; 
            }else if c == ')' {
                counter -= 1;
            }
            maxie = maxie.max(counter);
        }
        maxie
    }
}
