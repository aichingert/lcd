use crate::Solution;

impl Solution {
    pub fn check_valid_string(s: String) -> bool {
        let (mut l, mut r) = (0i32, 0);

        for c in s.chars() {
            match c {
                '(' => {
                    l += 1;
                    r += 1;
                }
                ')' => {
                    if r == 0 { return false; }
                    l = (l - 1).max(0);
                    r -= 1;
                }
                '*' => {
                    l = (l - 1).max(0);
                    r += 1;
                }
                _ => unreachable!(),
            }
        }

        l == 0
    }
}

