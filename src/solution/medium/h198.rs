use crate::Solution;

use std::collections::HashMap;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        find(&mut HashMap::new(), &nums, 0)
    }
}

fn find(memo: &mut HashMap<usize, i32>, n: &Vec<i32>, i: usize) -> i32 {
    if i >= n.len() {
        return 0;
    }
    if let Some(res) = memo.get(&i) {
        return *res;
    }

    let result = find(memo, n, i + 1).max(n[i] + find(memo, n, i + 2));
    memo.insert(i, result);
    result
}
