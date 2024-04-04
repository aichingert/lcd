use crate::Solution;

impl Solution {
    pub fn max_depth(s: String) -> i32 {
        let (mut n, mut m) = (0, 0);

        for c in s.chars() {
            match c {
                '(' => n += 1,
                ')' => n -= 1,
                _ => (),
            }

            m = m.max(n);
        }

        m
    }
}
