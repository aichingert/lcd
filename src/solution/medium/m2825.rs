use crate::Solution;

impl Solution {
    pub fn can_make_subsequence(str1: String, str2: String) -> bool {
        let s1 = str1.chars().collect::<Vec<_>>();
        let s2 = str2.chars().collect::<Vec<_>>();
        let mut i = 0;
        let mut j = 0;

        while i < s1.len() && j < s2.len() {
            let t = (s1[i] as u8) - b'a';
            let o = (s2[j] as u8) - b'a';
            let m = (t + 1) % 26;

            if o == t || o == m {
                i += 1;
                j += 1;
            } else {
                i += 1;
            }
        }

        j >= s2.len()
    }
}
