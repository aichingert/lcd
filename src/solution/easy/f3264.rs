use crate::Solution;

impl Solution {
    pub fn get_final_state(mut nums: Vec<i32>, k: i32, multiplier: i32) -> Vec<i32> {
        for _ in 0..k {
            let mut p = 0;
            let mut m = i32::MAX;

            for (i, &n) in nums.iter().enumerate() {
                if n < m {
                    m = n;
                    p = i;
                }
            }

            nums[p] *= multiplier;
        }

        nums
    }
}
