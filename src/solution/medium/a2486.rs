use crate::Solution;

impl Solution {
    pub fn append_characters(s: String, t: String) -> i32 {
        let (s, t) = (s.chars().collect::<Vec<_>>(), t.chars().collect::<Vec<_>>());
        let (mut sp, mut tp) = (0, 0);

        while sp < s.len() && tp < t.len() {
            if s[sp] == t[tp] {
                sp += 1;
                tp += 1;
            } else {
                sp += 1;
            }
        }

        (t.len() - tp) as i32
    }
}
