use crate::Solution;

impl Solution {
    pub fn largest_combination(candidates: Vec<i32>) -> i32 {
        let mut dp = [0; 32];

        for mut cand in candidates {
            let mut i = 0;

            while cand > 0 {
                i += 1;

                if cand & 1 == 1 {
                    dp[i - 1] += 1;
                }

                cand >>= 1;
            }
        }

        dp.into_iter().max().unwrap()
    }
}
