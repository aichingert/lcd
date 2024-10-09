use crate::Solution;

use std::collections::HashMap;

impl Solution {
    pub fn rob_lc_01(nums: Vec<i32>) -> i32 {
        (s(&mut HashMap::new(), &nums, 2, true, nums[0])).max(s(
            &mut HashMap::new(),
            &nums,
            1,
            false,
            0,
        ))
    }
}

fn s(m: &mut HashMap<(usize, bool), i32>, h: &[i32], c: usize, t: bool, a: i32) -> i32 {
    if c >= h.len() || c == h.len() - 1 && t {
        return a;
    }

    if let Some(&r) = m.get(&(c, t)) {
        if a + h[c] <= r {
            return a;
        }
    }

    m.insert((c, t), a + h[c]);

    (s(m, h, c + 2, t, a + h[c])).max(s(m, h, c + 1, t, a))
}
