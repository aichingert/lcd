use crate::Solution;

impl Solution {
    pub fn largest_submatrix(mut matrix: Vec<Vec<i32>>) -> i32 {
        let (n, m, mut ans) = (matrix.len(), matrix[0].len(), 0);

        for i in 1..n {
            for j in 0..m {
                println!("{:?} : {:?}", (i, j), matrix);
                if matrix[i][j] != 0 { matrix[i][j] += matrix[i - 1][j]; }
            }
        }


        for i in 0..n {
            matrix[i].sort_unstable();

            for j in 0..m {
                ans = ans.max((m - j) as i32 * matrix[i][j]);
            }
        }

        ans
    }
}
