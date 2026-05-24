
struct Solution;

impl Solution {
    pub fn min_time_to_type(word: String) -> i32 {
        let mut time = 0;
        let mut prev = 'a';
        for c in word.chars() {
            let diff = (c as i32 - prev as i32).abs();
            time += diff.min(26 - diff);
            prev = c;
        }
        time
    }
}
