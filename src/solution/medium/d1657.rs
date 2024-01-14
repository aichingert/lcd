use crate::Solution;

impl Solution {
    pub fn close_strings(word1: String, word2: String) -> bool {
        let (mut w1, mut w2) = ([0u8; 26], [0u8; 26]);

        for ch in word1.chars() {
            if !word2.contains(ch) {
                return false;
            }
            w1[(ch as u8 - b'a') as usize] += 1;
        }

        for ch in word2.chars() {
            w2[(ch as u8 - b'a') as usize] += 1;
        }

        w1.sort_unstable();
        w2.sort_unstable();

        w1 == w2
    }
}
