struct Solution;

impl Solution {
    fn gcd(a: i64, b: i64) -> i64 {
        if b == 0 { a } else { Self::gcd(b, a % b) }
    }

    pub fn count_pairs(nums: Vec<i32>, k: i32) -> i64 {
        let k = k as i64;
        let max_val = 100_001usize;

        let mut freq = vec![0i64; max_val];
        for &n in &nums {
            freq[n as usize] += 1;
        }

        let mut cnt = vec![0i64; (k + 1) as usize];
        let mut d = 1;
        while d * d <= k {
            if k % d == 0 {
                // d is a divisor of k — sum all multiples of d in freq
                let mut multiple = d as usize;
                while multiple < max_val {
                    cnt[d as usize] += freq[multiple];
                    multiple += d as usize;
                }
                // k/d is also a divisor
                let d2 = k / d;
                if d2 != d {
                    let mut multiple = d2 as usize;
                    while multiple < max_val {
                        cnt[d2 as usize] += freq[multiple];
                        multiple += d2 as usize;
                    }
                }
            }
            d += 1;
        }
        // ans += cnt[need], then subtract self-pair if nums[i] % need == 0
        //
        let mut ans: i64 = 0;
        for &n in &nums {
            let n = n as i64;
            let need = k / Self::gcd(k, n);
            ans += cnt[need as usize];
            if n % need == 0 {
                ans -= 1;
            }
        }

        ans / 2 // each pair (i,j) was counted as (i,j) and (j,i)
    }
}
