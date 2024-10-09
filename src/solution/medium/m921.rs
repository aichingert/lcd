use crate::Solution;

impl Solution {
    pub fn min_add_to_make_valid(s: String) -> i32 {
        let mut p = Vec::new();
        let mut a = 0;

        for c in s.chars() {
            if c == '(' {
                p.push(c);
            } else if p.pop().is_none() {
                a += 1;
            }
        }

        (a + p.len()) as i32
    }
}
