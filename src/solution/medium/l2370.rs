use crate::Solution;

impl Solution {
    pub fn longest_ideal_string(s: String, k: i32) -> i32 {
        let (k, mut dp) = (k as usize, [0; 256]);
        for b in s.bytes() {
            let b = b as usize;
            dp[b] = 1 + ((b - k)..=(b + k)).fold(0, |val, i| val.max(dp[i]));
        }
        dp.into_iter().max().unwrap()
    }
}
