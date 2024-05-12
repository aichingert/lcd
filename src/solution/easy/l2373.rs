use crate::Solution;

impl Solution {
    pub fn largest_local(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut ans = vec![vec![]; grid.len() - 2];

        for i in 1..grid.len() - 1 {
            for j in 1..grid[i].len() - 1 {
                let mut max = 0;

                for k in -1..2 {
                    for l in -1..2 {
                        max = grid[(i as i32 + k) as usize][(j as i32 + l) as usize].max(max);
                    }
                }

                ans[i - 1].push(max);
            }
        }

        ans
    }
}
