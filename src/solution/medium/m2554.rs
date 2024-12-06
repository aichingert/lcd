use crate::Solution;

impl Solution {
    pub fn max_count(mut banned: Vec<i32>, n: i32, max_sum: i32) -> i32 {
        banned.sort_unstable();
        let mut s = 0;
        let mut a = 0;

        for i in 1..=n {
            if s >= max_sum {
                break;
            }
            if banned.binary_search(&i).is_ok() {
                continue;
            }

            if s + i <= max_sum {
                s += i;
                a += 1;
            }
        }

        a
    }
}
