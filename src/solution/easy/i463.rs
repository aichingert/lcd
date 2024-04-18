use crate::Solution;

impl Solution {
    pub fn island_perimeter(grid: Vec<Vec<i32>>) -> i32 {
        let mut ans = 0;

        for m in 0..grid.len() {
            for n in 0..grid[m].len() {
                if grid[m][n] == 0 {
                    continue;
                }
                let (mi, ni) = (m as i32, n as i32);

                if mi - 1 < 0 || grid[m - 1][n] == 0 {
                    ans += 1;
                }
                if ni - 1 < 0 || grid[m][n - 1] == 0 {
                    ans += 1;
                }
                if m + 1 >= grid.len() || grid[m + 1][n] == 0 {
                    ans += 1;
                }
                if n + 1 >= grid[0].len() || grid[m][n + 1] == 0 {
                    ans += 1;
                }
            }
        }

        ans
    }
}
