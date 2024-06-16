use crate::Solution;

impl Solution {
    pub fn min_patches(nums: Vec<i32>, n: i32) -> i32 {
        let mut sum = 0u64;
        let nums = nums.into_iter().map(|n| n as u64).collect::<Vec<_>>();
        let n = n as u64;
        let mut ans = 0;
        let mut i = 0;

        while sum < n && i < nums.len() {
            if sum >= nums[i] || nums[i] == sum + 1 {
                sum += nums[i];
                i += 1;
            } else {
                ans += 1;
                sum = 2 * sum + 1;
            }
        }

        while sum < n {
            sum = 2 * sum + 1;
            ans += 1;
        }

        ans
    }
}
