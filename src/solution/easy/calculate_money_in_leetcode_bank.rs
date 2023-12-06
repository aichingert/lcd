use crate::Solution;

impl Solution {
    pub fn total_money(n: i32) -> i32 {
        let s = n / 7;
        let r = n - s * 7;

        28 * s + 7 * ((s * (s - 1)) / 2) + s * r + ((r + 1) * r) / 2
    }
}
