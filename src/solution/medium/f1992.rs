use crate::Solution;

use std::collections::HashSet;

impl Solution {
    pub fn find_farmland(land: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut s = HashSet::new();
        let mut ans = Vec::new();

        for i in 0..land.len() {
            for j in 0..land[i].len() {
                if land[i][j] == 0 || !s.insert((i, j)) {
                    continue;
                }

                let mut pos = vec![i as i32, j as i32, i as i32, j as i32];
                let mut bfs = vec![(i, j)];

                while let Some((y, x)) = bfs.pop() {
                    for (dy, dx) in [(0, 1), (1, 0)] {
                        let (y, x) = (y + dy, x + dx);

                        if y >= land.len() || x >= land[y].len() {
                            continue;
                        }

                        if land[y][x] == 1 && s.insert((y, x)) {
                            pos[2] = pos[2].max(y as i32);
                            pos[3] = pos[3].max(x as i32);
                            bfs.push((y, x));
                        }
                    }
                }

                ans.push(pos);
            }
        }

        ans
    }
}
