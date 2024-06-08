use crate::Solution;
use std::collections::HashSet;

impl Solution {
    pub fn check_subarray_sum(nums: Vec<i32>, k: i32) -> bool {
        let mut pre_sum;
        let mut old_sum = 0;
        let mut seen = HashSet::new();

        for n in nums {
            pre_sum = (old_sum + n) % k;

            if seen.contains(&pre_sum) {
                return true;
            }

            seen.insert(old_sum);
            old_sum = pre_sum;
        }

        false
    }
}
