use crate::Solution;

impl Solution {
    pub fn minimum_steps(s: String) -> i64 {
        let mut a = 0;
        let mut b = 0;

        for c in s.chars() {
            match c {
                '0' => {
                    a += b;
                }
                '1' => {
                    b += 1;
                }
                _ => panic!("{c}"),
            }
        }

        a
    }
}
