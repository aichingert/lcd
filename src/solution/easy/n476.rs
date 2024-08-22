use crate::Solution;

impl Solution {
    pub fn find_complement(n: i32) -> i32 {
        n ^ (i32::MAX >> (31.min(n.leading_zeros()) - 1))
    }
}
