use crate::Solution;

impl Solution {
    pub fn rotate_string(s: String, goal: String) -> bool {
        if s.len() != goal.len() {
            return false;
        }

        let s = s.chars().collect::<Vec<_>>();
        let g = goal.chars().collect::<Vec<_>>();

        'l: for i in 0..s.len() {
            for (j, &c) in g.iter().enumerate() {
                if s[(i + j) % s.len()] != c {
                    continue 'l;
                }
            }

            return true;
        }

        false
    }
}
