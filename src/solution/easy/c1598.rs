use crate::Solution;

impl Solution {
    pub fn min_operations_2_lc_l(logs: Vec<String>) -> i32 {
        let mut ans = 0;

        for log in logs {
            match log.as_str() {
                "../" => ans = 0.max(ans - 1),
                "./" => (),
                _ => ans += 1,
            }
        }

        ans
    }
}
