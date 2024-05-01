use crate::Solution;

impl Solution {
    pub fn reverse_prefix(word: String, ch: char) -> String {
        let k = word.find(ch).unwrap_or(0);
        word
            .bytes()
            .take(k + 1)
            .rev()
            .chain(word.bytes().skip(k + 1))
            .map(|u| u as char)
            .collect::<String>()
    }
}
