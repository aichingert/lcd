use crate::Solution;

impl Solution {
    pub fn min_increment_for_unique(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        let mut ans = 0;

        for i in 1..nums.len() {
            if nums[i] <= nums[i - 1] {
                let res = 1 + nums[i - 1] - nums[i];
                nums[i] += res;
                ans += res;
            }
        }

        ans
    }
}
