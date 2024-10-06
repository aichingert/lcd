use crate::Solution;

impl Solution {
    pub fn common_chars(words: Vec<String>) -> Vec<String> {
        let mut cnts = vec![[0; 26]; words.len()];

        for (i, w) in words.into_iter().enumerate() {
            for c in w.bytes() {
                cnts[i][(c - b'a') as usize] += 1;
            }
        }

        let res = cnts.into_iter().fold([1000; 26], |mut a, c| {
            for i in 0..a.len() {
                a[i] = a[i].min(c[i]);
            }
            a
        });
        let mut ans = Vec::new();

        for (i, a) in res.into_iter().enumerate() {
            for _ in 0..a {
                ans.push(((i as u8 + b'a') as char).to_string());
            }
        }

        ans
    }
}
