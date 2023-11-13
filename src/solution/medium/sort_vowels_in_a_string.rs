use crate::is_vowel;
use crate::Solution;

impl Solution {
    pub fn sort_vowels(s: String) -> String {
        let chs = s.chars().collect::<Vec<_>>();
        let mut vowels = chs.iter().filter(|&c| is_vowel(*c)).collect::<Vec<_>>();
        vowels.sort();
        
        let mut ans = String::new();
        let mut j = 0usize;

        for i in 0..chs.len() {
            ans.push(if is_vowel(chs[i]) {
                j += 1;
                *vowels[j - 1]
            } else {
                chs[i]
            });
        }

        ans
    }
}
