struct Solution;

impl Solution {
    pub fn longest_arithmetic(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n < 2 { return n as i32; }

        let mut l = vec![1i32; n];
        l[1] = 2;
        for i in 2..n {
            l[i] = if nums[i] - nums[i-1] == nums[i-1] - nums[i-2] { l[i-1] + 1 } else { 2 };
        }

        let mut r = vec![1i32; n];
        r[n-2] = 2;
        for i in (0..n-2).rev() {
            r[i] = if nums[i+1] - nums[i] == nums[i+2] - nums[i+1] { r[i+1] + 1 } else { 2 };
        }

        let mut ans = *l.iter().max().unwrap();

        for j in 0..n {
            if j + 1 < n {
                ans = ans.max(r[j+1] + 1);
            }
            if j > 0 {
                ans = ans.max(l[j-1] + 1);
            }
            if j > 0 && j + 1 < n {
                let needed = nums[j+1] - nums[j-1];
                if needed % 2 == 0 {
                    let d = needed / 2;
                    let left_len  = if j >= 2 && nums[j-1] - nums[j-2] == d { l[j-1] } else { 1 };
                    let right_len = if j < n - 2 && nums[j+2] - nums[j+1] == d { r[j+1] } else { 1 };
                    ans = ans.max(left_len + right_len + 1);
                }
            }
        }

        ans
    }
}
