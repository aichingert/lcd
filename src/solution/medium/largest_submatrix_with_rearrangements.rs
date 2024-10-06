use crate::Solution;

impl Solution {
    pub fn largest_submatrix(mut matrix: Vec<Vec<i32>>) -> i32 {
        let (n, m, mut ans) = (matrix.len(), matrix[0].len(), 0);

        for i in 1..n {
            for j in 0..m {
                if matrix[i][j] != 0 {
                    matrix[i][j] += matrix[i - 1][j];
                }
            }
        }

        for row in matrix.iter_mut().take(n) {
            row.sort_unstable();

            for (i, col) in row.iter().enumerate() {
                ans = ans.max((m - i) as i32 * col);
            }
        }

        ans
    }
}
