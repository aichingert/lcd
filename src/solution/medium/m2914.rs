use crate::Solution;

impl Solution {
    pub fn min_changes(s: String) -> i32 {
        let c = s.chars().collect::<Vec<_>>();
        let mut a = 0;

        for i in (0..c.len()).step_by(2) {
            if c[i] != c[i + 1] {
                a += 1;
            }
        }

        a
    }
}
