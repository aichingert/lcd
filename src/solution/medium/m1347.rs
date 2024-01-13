use crate::Solution;

impl Solution {
    pub fn min_steps(s: String, t: String) -> i32 {
        let mut chs = [0i32; 26];

        for a in s.chars() {
            chs[(a as u8 - b'a') as usize] += 1;
        }
        for b in t.chars() {
            chs[(b as u8 - b'a') as usize] -= 1;
        }

        chs.iter().map(|n| n.abs()).sum::<i32>() / 2
    }
}
