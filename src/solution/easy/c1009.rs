// NOTE: there are no tests for this file as it is the same as n476.rs

impl Solution {
    pub fn bitwise_complement(n: i32) -> i32 {
        n ^ (i32::MAX >> (31.min(n.leading_zeros()) - 1))
    }
}
