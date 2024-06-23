use crate::Solution;

use std::collections::BTreeMap;

impl Solution {
    pub fn longest_subarray(nums: Vec<i32>, limit: i32) -> i32 {
        let mut map = BTreeMap::new();
        let mut ans = 0;
        let mut l = 0;

        for r in 0..nums.len() {
            *map.entry(nums[r]).or_insert(0) += 1;

            let (mut min, mut max) = (*map.first_key_value().unwrap().0, *map.last_key_value().unwrap().0);

            match max - min <= limit {
                true => ans = ans.max(r - l + 1),
                false => {
                    while max - min > limit {
                        let e = map.get_mut(&nums[l]).unwrap();
                        *e -= 1;
                        if *e == 0 {
                            map.remove(&nums[l]);
                        }
                        (max, min) = (*map.first_key_value().unwrap().0, *map.last_key_value().unwrap().0);
                        l += 1;
                    }
                }
            }
        }

        ans as i32
    }
}
