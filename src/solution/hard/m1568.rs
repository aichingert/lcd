use crate::Solution;

use std::collections::HashSet;

impl Solution {
    pub fn min_days_lc_1(mut grid: Vec<Vec<i32>>) -> i32 {
        let (s, p) = get_islands(&grid);

        if s != 1 {
            return 0;
        }

        for (y, x) in p {
            grid[y as usize][x as usize] = 0;

            if get_islands(&grid).0 != 1 {
                return 1;
            }

            grid[y as usize][x as usize] = 1;
        }

        2
    }
}

fn get_islands(g: &[Vec<i32>]) -> (i32, HashSet<(i32, i32)>) {
    let mut v = HashSet::new();
    let mut s = Vec::<(i32, i32)>::new();
    let mut l = 0;

    for i in 0..g.len() {
        for j in 0..g[i].len() {
            if g[i][j] == 0 { continue; }

            if !v.contains(&(i as i32, j as i32)) {
                l += 1;
            }

            s.push((i as i32, j as i32));

            while let Some((y, x)) = s.pop() {
                if x < 0 || y < 0 || y >= g.len() as i32 || x >= g[i].len() as i32{
                    continue;
                }
                if g[y as usize][x as usize] == 0 {
                    continue;
                }
                if !v.insert((y, x)) {
                    continue;
                }

                s.push((y + 1, x));
                s.push((y - 1, x));
                s.push((y, x + 1));
                s.push((y, x - 1));
            }
        }
    }

    (l, v)
}
