const MOD: u64 = 10u64.pow(9) + 7;

use crate::Solution;

impl Solution {
    pub fn knight_dialer(n: i32) -> i32 {
        let mut ans = [1u64; 10];

        for _ in 1..n {
            let mut next = [0; 10];
            
            next[0] = (ans[5] + ans[7]) % MOD;
            next[1] = (ans[6] + ans[8]) % MOD;
            next[2] = (ans[3] + ans[7]) % MOD;
            next[3] = (ans[2] + ans[8] + ans[9]) % MOD;
            next[5] = (ans[0] + ans[6] + ans[9]) % MOD;
            next[6] = (ans[1] + ans[5]) % MOD;
            next[7] = (ans[0] + ans[2]) % MOD;
            next[8] = (ans[3] + ans[1]) % MOD;
            next[9] = (ans[3] + ans[5]) % MOD;

            ans = next;
        }

        (ans.iter().sum::<u64>() % MOD) as i32
    }
}
