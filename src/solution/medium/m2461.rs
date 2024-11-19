use crate::Solution;

use std::collections::HashMap;

impl Solution {
    pub fn maximum_subarray_sum(nums: Vec<i32>, k: i32) -> i64 {
        if k == 1 {
            return nums.into_iter().max().unwrap() as i64;
        }

        let k = k as usize;
        let mut a = 0;
        let (mut s, mut m) = (0, HashMap::new());
        let (mut l, mut r) = (0, k);

        for &n in nums.iter().take(k) {
            s += n as i64;
            m.entry(n).and_modify(|n| *n += 1).or_insert(1);
        }

        while r < nums.len() {
            if m.len() == k {
                a = a.max(s);
            }

            s -= nums[l] as i64;
            s += nums[r] as i64;

            if let Some(c) = m.remove(&nums[l]) {
                if c > 1 {
                    m.insert(nums[l], c - 1);
                }
            }

            m.entry(nums[r]).and_modify(|n| *n += 1).or_insert(1);
            l += 1;
            r += 1;
        }

        if m.len() == k {
            a = a.max(s);
        }

        a
    }
}
