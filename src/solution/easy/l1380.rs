use crate::Solution;

impl Solution {
    pub fn lucky_numbers(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let mut ans: Vec<Result<i32, i32>> = vec![Err(-1); matrix[0].len()];

        for row in &matrix {
            let mut min = i32::MAX;
            let mut idx = 0;

            for (j, a) in ans.iter_mut().enumerate().take(row.len()) {
                if match *a {
                    Ok(n) => n,
                    Err(n) => n,
                } < row[j]
                {
                    *a = Err(row[j]);
                }

                if row[j] < min {
                    min = row[j];
                    idx = j;
                }
            }

            if match ans[idx] {
                Ok(n) => n,
                Err(n) => n,
            } <= min
            {
                ans[idx] = Ok(min);
            }
        }

        ans.into_iter().flatten().collect()
    }
}
