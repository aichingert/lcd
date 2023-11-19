use crate::Solution;

impl Solution {
    pub fn reduction_operations(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        let mut ans = 0;
        let mut sum = 0;
        let mut cur = 0;

        for i in (1..nums.len()).rev() {
            cur += 1;

            if nums[i] != nums[i  - 1] {
                sum += cur;
                ans += sum;
                cur = 0;

                if nums[i] == nums[0] {
                    break;
                }
            }
        }

        ans
    }
}
