use crate::Solution;

// TODO: TLE
impl Solution {
    pub fn find_the_longest_substring(s: String) -> i32 {
        let mut a = 0;
        let mut p = Vec::new();
        let mut state;

        for c in s.bytes() {
            if matches!(c, b'a' | b'e' | b'o' | b'i' | b'u') {
                state = 1 << (c - b'a');
            } else {
                state = 0;
            }

            p.push(state);
        }


        for i in 0..p.len() {
            state = 0;

            for (j, e) in p.iter().enumerate().skip(i) {
                state ^= e;

                if state == 0 {
                    a = a.max(1 + j - i);
                }
            }
        }

        a as i32
    }
}
