use crate::Solution;

use std::cmp::Ordering;

impl Solution {
    pub fn num_subarrays_with_sum(nums: Vec<i32>, goal: i32) -> i32 {
        let mut ans = 0;

        for i in 0..nums.len() {
            let mut s = 0;

            for j in i..nums.len() {
                s += nums[j];

                match s.cmp(&goal) {
                    Ordering::Less => (),
                    Ordering::Equal => ans += 1,
                    Ordering::Greater => break,
                }
            }
        }

        ans
    }
}

