use crate::Solution;

use std::collections::HashMap;

impl Solution {
    pub fn max_subarray_length(nums: Vec<i32>, k: i32) -> i32 {
        let mut f = HashMap::new();
        let mut ans = 0;

        let (mut l, mut r) = (0, 0);

        while r < nums.len() {
            f.entry(nums[r]).and_modify(|n| *n += 1).or_insert(1);

            while f[&nums[r]] > k {
                *f.get_mut(&nums[l]).unwrap() -= 1;
                l += 1;
            }

            ans = ans.max(r - l + 1);

            r += 1;
        }

        ans as i32
    }
}
