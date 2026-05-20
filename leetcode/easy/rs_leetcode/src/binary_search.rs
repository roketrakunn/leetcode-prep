struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        Self::helper(&nums, target, 0)
    }

    fn helper(nums: &[i32], target: i32, offset: usize) -> i32 {
        if nums.is_empty() {
            return -1;
        }
        let mid = nums.len() / 2;
        if target == nums[mid] {
            return (offset + mid) as i32;
        } else if target < nums[mid] {
            Self::helper(&nums[..mid], target, offset)
        } else {
            Self::helper(&nums[mid + 1..], target, offset + mid + 1)
        }
    }
}
