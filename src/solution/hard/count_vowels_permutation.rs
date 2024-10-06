/* Daily problem: 2023-10-28
 * aichingert
 */

use crate::Solution;

impl Solution {
    pub fn count_vowel_permutation(n: i32) -> i32 {
        let mut start = [1; 5];
        const MOD: i64 = 1e9 as i64 + 7;

        for _ in 1..n {
            let mut cp = [0; 5];

            cp[0] = (start[1] + start[2] + start[4]) % MOD;
            cp[1] = (start[0] + start[2]) % MOD;
            cp[2] = (start[1] + start[3]) % MOD;
            cp[3] = start[2];
            cp[4] = (start[2] + start[3]) % MOD;

            start = cp;
        }

        (start.iter().sum::<i64>() % MOD) as i32
    }
}
