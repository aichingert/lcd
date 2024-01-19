use crate::Solution;

use std::collections::HashMap;

impl Solution {
    pub fn min_falling_path_sum(matrix: Vec<Vec<i32>>) -> i32 {
        let mut ans = i32::MAX;
        let mut memo = HashMap::new();

        for i in 0..matrix.len() {
            ans = ans.min(dfs(&mut memo, &matrix, (0, i)));
        }

        ans
    }
}

fn dfs(memo: &mut HashMap<(usize, usize), i32>, m: &Vec<Vec<i32>>, (r, c): (usize, usize)) -> i32 {
    if let Some(res) = memo.get(&(r, c)) {
        return *res;
    }
    
    if r >= m.len() || c >= m[0].len() {
        return i32::MAX;
    }

    let mut below = dfs(memo, m, (r + 1, c));

    if c > 0 {
        below = below.min(dfs(memo, m, (r + 1, c - 1)));
    }

    if c < m[r].len() {
        below = below.min(dfs(memo, m, (r + 1, c + 1)));
    }

    let res = if below != i32::MAX {
        m[r][c] + below
    } else {
        m[r][c]
    };
    memo.insert((r, c), res);
    res
}
