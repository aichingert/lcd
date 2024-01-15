use crate::Solution;

const LETTERS: usize = 26;

impl Solution {
    pub fn count_palindromic_subsequence(s: String) -> i32 {
        let chs = s.chars().collect::<Vec<_>>();
        let mut first = [s.len(); LETTERS];
        let mut last = [0; LETTERS];

        for (i, ch) in chs.iter().enumerate() {
            let idx = (*ch as u8 - b'a') as usize;
            first[idx] = first[idx].min(i);
            last[idx] = last[idx].max(i);
        }

        let mut ans = 0;

        for i in 0..LETTERS {
            let mut letters = 0u32;

            for ch in chs.iter().take(last[i]).skip(first[i] + 1) {
                if letters.count_ones() == LETTERS as u32 {
                    break;
                }

                letters |= 1 << (*ch as u8 - b'a');
            }

            ans += letters.count_ones() as i32;
        }

        ans
    }
}
