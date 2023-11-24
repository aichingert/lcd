use crate::Solution;

impl Solution {
    pub fn min_pair_sum(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        let mut ans = 0;

        for i in 0..nums.len() / 2 {
            ans = ans.max(nums[i] + nums[nums.len() - i - 1]);
        }

        ans
    }
}
