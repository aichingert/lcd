use crate::Solution;

impl Solution {
    pub fn max_matrix_sum(matrix: Vec<Vec<i32>>) -> i64 {
        let mut s = 0i64;
        let mut n = 0;
        let mut m = i64::MAX;

        for r in &matrix {
            for &v in r {
                if v < 0 {
                    n += 1;
                }
                let abs = v.abs() as i64;

                m = m.min(abs);
                s += abs;
            }
        }

        if n & 1 == 1 {
            s += -m * 2;
        }

        s
    }
}
