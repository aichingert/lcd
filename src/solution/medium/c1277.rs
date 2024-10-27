use crate::Solution;

impl Solution {
    pub fn count_squares(matrix: Vec<Vec<i32>>) -> i32 {
        let mut res = 0;

        for i in 0..matrix.len() {
            for j in 0..matrix[i].len() {
                if matrix[i][j] == 0 {
                    continue;
                }

                res += 1;
                let mut dx = j;
                let mut dy = i;

                'chk: while dy + 1 < matrix.len() && dx + 1 < matrix[dy].len() {
                    dx += 1;
                    dy += 1;

                    for r in matrix.iter().take((dy - 1) + 1).skip(i) {
                        if r[dx] == 0 {
                            break 'chk;
                        }
                    }

                    for p in j..=dx {
                        if matrix[dy][p] == 0 {
                            break 'chk;
                        }
                    }

                    res += 1;
                }
            }
        }

        res
    }
}
