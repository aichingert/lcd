use crate::Solution;

impl Solution {
    pub fn score_of_string(s: String) -> i32 {
        s.chars()
            .fold((0i32, 0i32), |acc, n| {
                let ch = (n as u8) as i32;
                let sum = if acc.1 != 0 {
                    acc.0 + (acc.1 - ch).abs()
                } else {
                    0
                };

                (sum, ch)
            })
            .0
    }
}
