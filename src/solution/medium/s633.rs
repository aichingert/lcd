use crate::Solution;

impl Solution {
    pub fn judge_square_sum(c: i32) -> bool {
        let sqrs = (0..(i32::MAX as f64).sqrt().floor() as i32)
            .map(|i| i * i)
            .collect::<Vec<_>>();

        for i in 0..sqrs.len() {
            if sqrs[i] > c {
                break;
            }

            if c == sqrs[i] || sqrs.binary_search(&(c - sqrs[i])).is_ok() {
                return true;
            }
        }

        false
    }
}
