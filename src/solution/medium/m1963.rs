use crate::Solution;

impl Solution {
    pub fn min_swaps_lc_01(s: String) -> i32 {
        let mut p = Vec::new();
        let mut u = 0;

        for c in s.chars() {
            if c == '[' {
                p.push(c);
            } else if p.pop().is_none() {
                u += 1;
            }
        }
        
        (u + 1) / 2
    }
}
