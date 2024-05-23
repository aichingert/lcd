use crate::Solution;

use std::collections::HashMap;

impl Solution {
    pub fn beautiful_subsets(nums: Vec<i32>, k: i32) -> i32 {
        sub(&nums, k, 0, &mut HashMap::new()) - 1
    }
}

fn sub(xs: &[i32], k: i32, i: usize, val: &mut HashMap<i32, i32>) -> i32 {
    if i >= xs.len() {
        return 1;
    }

    let mut ans = 0;

    ans += sub(xs, k, i + 1, val);

    if *val.get(&(xs[i] - k)).unwrap_or(&0) == 0 && *val.get(&(xs[i] + k)).unwrap_or(&0) == 0 {
        val.entry(xs[i]).and_modify(|n| *n += 1).or_insert(1);
        ans += sub(xs, k, i + 1, val);
        *val.get_mut(&(xs[i])).unwrap() -= 1;
    }

    ans
}

