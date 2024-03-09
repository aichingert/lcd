use std::collections::HashMap;

use crate::Solution;

impl Solution {
    pub fn max_frequency_elements(nums: Vec<i32>) -> i32 {
        let mut m = HashMap::new();

        for n in nums {
            m.entry(n).and_modify(|x| *x += 1).or_insert(1);
        }

        let ma = *m.values().max().unwrap();
        let mut ans = 0;

        for (_, k) in m {
            if k == ma {
                ans += k;
            }
        }

        ans
    }
}
