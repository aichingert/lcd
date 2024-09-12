use crate::Solution;

impl Solution {
    pub fn count_consistent_strings(allowed: String, words: Vec<String>) -> i32 {
        let s = allowed.bytes().fold(0, |acc, nxt| acc | 1 << (nxt - b'a'));
        let mut a = 0;

        'lp: for w in words {
            for c in w.bytes() {
                if s & 1 << (c - b'a') == 0 {
                    continue 'lp;
                }
            }

            a += 1;
        }

        a
    }
}
