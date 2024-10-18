use crate::Solution;

impl Solution {
    pub fn count_max_or_subsets(nums: Vec<i32>) -> i32 {
        let m = nums.iter().fold(0, |acc, c| acc | c);
        sol(&nums, 0, m)
    }
}

fn sol(n: &[i32], cur: i32, m: i32) -> i32 {
    if n.is_empty() {
        return if cur == m { 1 } else { 0 };
    }

    sol(&n[1..], cur | n[0], m) + sol(&n[1..], cur, m)
}
