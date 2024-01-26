use crate::Solution;
use std::collections::HashMap;

const MOD: i64 = 10000000007;

impl Solution {
    pub fn find_paths(m: i32, n: i32, max_move: i32, start_row: i32, start_column: i32) -> i32 {
        (dfs(&mut HashMap::new(), start_column, start_row, m, n, 0, max_move) % MOD) as i32
    }
}

fn dfs(memo: &mut HashMap<(i32, i32, i32), i64>, x: i32, y: i32, m: i32, n: i32, mut cur: i32, max: i32) -> i64 {
    if x < 0 || y < 0 || x >= n || y >= m {
        return 1;
    }

    if cur >= max {
        return 0;
    }

    if let Some(res) = memo.get(&(x, y, cur)) {
        return *res;
    }

    cur += 1;
    let mut res = dfs(memo, x + 1, y, m, n, cur, max) % MOD;
    res = (res + dfs(memo, x - 1, y, m, n, cur, max) % MOD);
    res = (res + dfs(memo, x, y + 1, m, n, cur, max) % MOD);
    res = (res + dfs(memo, x, y - 1, m, n, cur, max) % MOD);
    res %= MOD;

    memo.insert((x, y, cur - 1), res);
    res
}
