use crate::Solution;

impl Solution {
    pub fn count_sub_islands(grid1: Vec<Vec<i32>>, grid2: Vec<Vec<i32>>) -> i32 {
        let mut v = vec![vec![0; grid1[0].len()]; grid1.len()];
        let mut a = 0;

        for i in 0..grid1.len() {
            for j in 0..grid1[i].len() {
                if v[i][j] != 0 || grid1[i][j] == 0 || grid2[i][j] == 0 {
                    continue;
                }

                let mut s = vec![(i, j)];
                let mut c = true;

                while let Some((y, x)) = s.pop() {
                    if v[y][x] == 1 || grid2[y][x] == 0 {
                        continue;
                    }
                    v[y][x] = 1;
                    if grid1[y][x] == 0 {
                        c = false;
                    }

                    if y > 0 {
                        s.push((y - 1, x));
                    }
                    if x > 0 {
                        s.push((y, x - 1));
                    }
                    if y + 1 < grid1.len() {
                        s.push((y + 1, x));
                    }
                    if x + 1 < grid1[y].len() {
                        s.push((y, x + 1));
                    }
                }

                if c { a += 1; }
            }
        }

        a
    }
}
