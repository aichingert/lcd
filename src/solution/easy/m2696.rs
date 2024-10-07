use crate::Solution;

impl Solution {
    pub fn min_length(s: String) -> i32 {
        let mut s = s.chars().collect::<Vec<_>>();
        let mut n = Vec::new();

        while let Some(c) = s.pop() {
            if !n.is_empty()
                && ((c == 'C' && n[n.len() - 1] == 'D') || (c == 'A' && n[n.len() - 1] == 'B'))
            {
                n.pop();
            } else {
                n.push(c);
            }
        }

        n.len() as i32
    }
}
