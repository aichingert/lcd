use crate::Solution;

impl Solution {
    pub fn subset_xor_sum(nums: Vec<i32>) -> i32 {
        sum(&nums, 0)
    }
}

fn sum(a: &[i32], c: i32) -> i32 {
    let mut ans = 0;

    for i in 0..a.len() {
        let n = c ^ a[i];
        ans += n + sum(&a[i + 1..], n);
    }

    ans
}
