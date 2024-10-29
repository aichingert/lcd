use crate::Solution;

use std::collections::BinaryHeap;
use std::collections::HashMap;

impl Solution {
    pub fn max_moves(grid: Vec<Vec<i32>>) -> i32 {
        let mut m = HashMap::new();
        let mut h = BinaryHeap::new();
        let mut ans = 0;

        for i in 0..grid.len() {
            h.push((0, (i, 0)));
        }

        while let Some((s, (r, c))) = h.pop() {
            ans = ans.max(s);

            if c + 1 >= grid[r].len()
                || m.get(&(r, c))
                    .and_then(|&ms| if ms <= s { Some(0) } else { None })
                    .is_some()
            {
                continue;
            }

            m.insert((r, c), s);

            if r > 0 && grid[r][c] < grid[r - 1][c + 1] {
                h.push((s + 1, (r - 1, c + 1)));
            }
            if r + 1 < grid.len() && grid[r][c] < grid[r + 1][c + 1] {
                h.push((s + 1, (r + 1, c + 1)));
            }
            if grid[r][c] < grid[r][c + 1] {
                h.push((s + 1, (r, c + 1)));
            }
        }

        ans
    }
}
