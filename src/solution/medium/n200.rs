use crate::Solution;

use std::collections::{HashSet, VecDeque};

impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let mut s = HashSet::new();
        let mut ans = 0;

        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                if grid[i][j] == '0' || s.contains(&(i, j)) {
                    continue;
                }
                ans += 1;

                let mut bfs = VecDeque::from_iter([(i, j)]);

                while let Some((dy, dx)) = bfs.pop_front() {
                    for (y, x) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
                        let (ny, nx) = (dy as i32 + y, dx as i32 + x);

                        if ny < 0 || nx < 0 || ny >= grid.len() as i32 || nx >= grid[0].len() as i32
                        {
                            continue;
                        }

                        let (y, x) = (ny as usize, nx as usize);
                        if grid[y][x] == '1' && s.insert((y, x)) {
                            bfs.push_back((y, x));
                        }
                    }
                }
            }
        }

        ans
    }
}
