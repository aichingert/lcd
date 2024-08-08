use crate::Solution;

impl Solution {
    pub fn spiral_matrix_iii(rows: i32, cols: i32, r_start: i32, c_start: i32) -> Vec<Vec<i32>> {
        let mut ans = Vec::new();
        
        let mut y = r_start;
        let mut x = c_start;
        let mut n = 1;

        while ans.len() as i32 != rows * cols {
            for _ in 0..n {
                if x >= 0 && x < cols && y >= 0 && y < rows {
                    ans.push(vec![y, x]);
                }

                x += 1;
            }

            for _ in 0..n {
                if x >= 0 && x < cols && y >= 0 && y < rows {
                    ans.push(vec![y, x]);
                }

                y += 1;
            }

            n += 1;

            for _ in 0..n {
                if x >= 0 && x < cols && y >= 0 && y < rows {
                    ans.push(vec![y, x]);
                }

                x -= 1;
            }

            for _ in 0..n {
                if x >= 0 && x < cols && y >= 0 && y < rows {
                    ans.push(vec![y, x]);
                }

                y -= 1;
            }

            n += 1;
        }

        ans
    }
}
