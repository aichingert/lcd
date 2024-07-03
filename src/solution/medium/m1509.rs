use crate::Solution;

impl Solution {
    pub fn min_difference(mut nums: Vec<i32>) -> i32 {
        if nums.len() <= 4 {
            return 0;
        }

        nums.sort_unstable();
        let mut ans = i32::MAX;

        for i in 0..4 {
            ans = ans.min(nums[nums.len() + i - 4] - nums[i]);
        }

        ans
    }
}
