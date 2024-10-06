use crate::Solution;

impl Solution {
    pub fn matrix_score(mut grid: Vec<Vec<i32>>) -> i32 {
        for r in grid.iter_mut() {
            if r[0] == 0 {
                for c in r.iter_mut() {
                    *c = 1 - *c;
                }
            }
        }

        for i in 1..grid[0].len() {
            let mut c = [0; 2];

            for j in 0..grid.len() {
                c[1 - grid[j][i] as usize] += 1;
            }

            if c[1] > c[0] {
                for r in grid.iter_mut() {
                    r[i] = 1 - r[i];
                }
            }
        }

        let mut ans = 0;
        for r in grid.iter_mut() {
            for i in 0..r.len() {
                if r[i] == 0 {
                    continue;
                }
                ans += 2_i32.pow((r.len() - i - 1) as u32);
            }
        }

        ans
    }
}
