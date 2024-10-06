use crate::Solution;

impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        use std::collections::HashMap;
        let mut counter = HashMap::new();
        nums.into_iter().for_each(|n| {
            counter.entry(n).and_modify(|x| *x += 1).or_insert(1);
        });

        let mut ans = 0;
        for (_, e) in counter {
            if e == 1 {
                return -1;
            }
            ans += (e + 2) / 3;
        }
        ans
    }
}
