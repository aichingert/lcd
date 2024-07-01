use crate::Solution;

impl Solution {
    pub fn three_consecutive_odds(arr: Vec<i32>) -> bool {
        let mut cnt = 0;

        for x in arr {
            if x & 1 == 1 {
                cnt += 1;
            } else {
                cnt = 0;
            }

            if cnt == 3 {
                return true;
            }
        }

        false
    }
}
