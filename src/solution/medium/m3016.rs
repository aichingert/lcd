use crate::Solution;

impl Solution {
    pub fn minimum_pushes(word: String) -> i32 {
        let mut m = [0; 26];

        for c in word.bytes() {
            m[(c - b'a') as usize] += 1;
        }

        m.sort_unstable_by(|a, b| b.cmp(a));

        let mut c = 1;
        let mut a = 0;

        for (i, e) in m.into_iter().enumerate() {
            if e == 0 { break; }
            if i > 0 && i % 8 == 0 {
                c += 1;
            }

            a += e * c;
        }

        a
    }
}
