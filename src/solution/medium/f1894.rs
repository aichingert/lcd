use crate::Solution;

impl Solution {
    pub fn chalk_replacer(chalk: Vec<i32>, k: i32) -> i32 {
        let mut a = 0i64;

        for &c in chalk.iter() {
            a += c as i64;
        }

        a = k as i64 % a;

        for (i, c) in chalk.into_iter().enumerate() {
            if a < c as i64 {
                return i as i32;
            }
            a -= c as i64;
        }

        unreachable!()
    }
}
