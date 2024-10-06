use crate::Solution;

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let mut dp = vec![0, 1];

        for i in 2..=n as usize + 1 {
            dp.push(dp[i - 1] + dp[i - 2]);
        }

        dp[(n + 1) as usize]
    }
}
