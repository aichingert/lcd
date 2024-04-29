use crate::Solution;

impl Solution {
    pub fn min_operations_1_lc_l(nums: Vec<i32>, k: i32) -> i32 {
        (nums.into_iter().fold(0, |acc, cur| acc ^ cur) ^ k).count_ones() as i32
    }
}
