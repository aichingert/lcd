use crate::Solution;
use std::collections::{HashSet, VecDeque};

impl Solution {
    pub fn get_maximum_gold(grid: Vec<Vec<i32>>) -> i32 {
        let mut bfs = VecDeque::new();

        for (i, r) in grid.iter().enumerate() {
            for (j, c) in r.iter().enumerate() {
                if *c > 0 {
                    bfs.push_back((i, j, *c, HashSet::from([(i, j)])));
                }
            }
        }

        let mut ans = 0;

        while let Some((i, j, g, v)) = bfs.pop_front() {
            ans = ans.max(g);

            for (dy, dx) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
                let (di, dj) = (i as i32 + dy, j as i32 + dx);

                if di < 0 || dj < 0 || di >= grid.len() as i32 || dj >= grid[0].len() as i32 {
                    continue;
                }

                let (di, dj) = (di as usize, dj as usize);
                let mut v = v.clone();
                if grid[di][dj] == 0 || !v.insert((di, dj)) {
                    continue;
                }
                bfs.push_back((di, dj, g + grid[di][dj], v));
            }
        }

        ans
    }
}
