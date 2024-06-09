use crate::Solution;

use std::collections::HashMap;

impl Solution {
    pub fn subarrays_div_by_k(nums: Vec<i32>, k: i32) -> i32 {
        let mut sum = 0;
        let mut ans = 0;
        let mut hm: HashMap<i32, i32> = HashMap::from_iter([(0, 1)]);

        for n in nums {
            sum += n;
            let rem = sum.rem_euclid(k);

            ans += *hm.get(&rem).unwrap_or(&0);
            hm.entry(rem).and_modify(|n| *n += 1).or_insert(1);
        }

        ans
    }
}
