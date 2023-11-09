use crate::Solution;

const MOD: usize = 1_000_000_000 + 7;

impl Solution {
    pub fn count_homogenous(s: String) -> i32 {
        let chs = s.chars().collect::<Vec<char>>();

        let mut s = 0;
        let mut i = 0;

        while chs.len() > i {
            let mut j = i + 1;

            while chs.len() > j && chs[i] == chs[j] {
                j += 1;
            }

            let n = (j - i) % MOD;
            s += (((n * (n + 1)) / 2) % MOD) as i32;
            i = j;
        }

        s
    }
}
