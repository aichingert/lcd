use crate::Solution;

impl Solution {
    pub fn restore_matrix(mut row_sum: Vec<i32>, mut col_sum: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ans = vec![vec![0; col_sum.len()]; row_sum.len()];
        let (mut i, mut j) = (0, 0);

        while i < row_sum.len() && j < col_sum.len() {
            let min = row_sum[i].min(col_sum[j]);
            ans[i][j] = min;

            if min == row_sum[i] {
                col_sum[j] -= min;
                i += 1;
            } else {
                row_sum[i] -= min;
                j += 1;
            }
        }

        ans
    }
}
