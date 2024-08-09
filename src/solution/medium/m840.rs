use crate::Solution;

impl Solution {
    pub fn num_magic_squares_inside(grid: Vec<Vec<i32>>) -> i32 {
        let mut ans = 0;

        for i in 0..grid.len() {
            'col: for j in 0..grid[i].len() {
                if j + 3 > grid[i].len() || i + 3 > grid.len() {
                    continue 'col;
                }

                let mut n = 0u32;
                let mut h = [0, 0, 0];
                let mut v = [0, 0, 0];
                let mut d = [0, 0];

                for k in i..i + 3 {
                    for l in j..j + 3 {
                        let cur = 1 << grid[k][l];
                        if grid[k][l] > 9 || n & cur != 0 {
                            continue 'col;
                        }

                        h[k - i] += grid[k][l];
                        v[k - i] += grid[i + l - j][j];

                        n |= cur;
                    }

                    d[0] += grid[k][j + k - i];
                    d[1] += grid[k][j + 2 - (k - i)];
                }

                if h[0] == h[1] && h[1] == h[2] && h[0] == 15 && 
                v[0] == v[1] && v[1] == v[2] && v[0] == 15 && 
                d[0] == d[1] && d[0] == 15 {
                    ans += 1;
                }
            }
        }

        ans
    }
}
