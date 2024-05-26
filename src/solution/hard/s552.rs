use crate::Solution;

use std::collections::HashMap;

const MOD: i64 = 10i64.pow(9) + 7;

// TODO: this solution is not optimal
// consider using an array [[i32; 2]; 3] for dp
impl Solution {
    pub fn check_record(n: i32) -> i32 {
        (brt(&mut HashMap::new(), n, 0, 0) % MOD) as i32
    }
}

fn brt(memo: &mut HashMap<(i32, i32, i32), i64>, n: i32, a: i32, l: i32) -> i64 {
    if n == 0 {
        return 1;
    }
    if let Some(r) = memo.get(&(n, a, l)) {
        return *r;
    }

    let mut ans = brt(memo, n - 1, a, 0);
    ans %= MOD;

    if a == 0 {
        ans += brt(memo, n - 1, a + 1, 0);
        ans %= MOD;
    }
    if l < 2 {
        ans += brt(memo, n - 1, a, l + 1);
        ans %= MOD;
    }

    memo.insert((n, a, l), ans);
    ans
}
