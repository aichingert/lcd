use crate::Solution;

use std::collections::HashSet;

impl Solution {
    pub fn regions_by_slashes(grid: Vec<String>) -> i32 {
        let n = grid.len();
        let mut t = vec![vec![0; n * 3]; n * 3];

        for (i, r) in grid.iter().enumerate() {
            for (j, c) in r.chars().enumerate() {
                match c {
                    '/' => {
                        t[i * 3][j * 3 + 2] = 1;
                        t[i * 3 + 1][j * 3 + 1] = 1;
                        t[i * 3 + 2][j * 3] = 1;
                    }
                    '\\' => {
                        t[i * 3][j * 3] = 1;
                        t[i * 3 + 1][j * 3 + 1] = 1;
                        t[i * 3 + 2][j * 3 + 2] = 1;
                    }
                    _ => (),
                }
            }
        }

        let mut v = HashSet::new();
        let mut a = 0;

        for (i, r) in t.iter().enumerate() {
            for (j, &c) in r.iter().enumerate() {
                if c == 1 {
                    continue;
                }

                if !v.contains(&(i as i32, j as i32)) {
                    a += 1;
                }

                let mut s = vec![(i as i32, j as i32)];

                while let Some((y, x)) = s.pop() {
                    if !(y > -1 && y < t.len() as i32 && x > -1 && x < r.len() as i32) {
                        continue;
                    }
                    if !v.insert((y, x)) {
                        continue;
                    }
                    if t[y as usize][x as usize] == 1 {
                        continue;
                    }

                    s.push((y, x - 1));
                    s.push((y, x + 1));
                    s.push((y - 1, x));
                    s.push((y + 1, x));
                }
            }
        }

        a
    }
}
