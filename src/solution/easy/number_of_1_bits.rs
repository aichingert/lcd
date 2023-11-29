use crate::Solution;

impl Solution {
    pub fn hamming_weight(n: u32) -> i32 {
        n.count_ones() as i32
    }
}
