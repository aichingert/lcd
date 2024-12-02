use crate::Solution;

impl Solution {
    pub fn is_prefix_of_word(sentence: String, search_word: String) -> i32 {
        for (i, word) in sentence.split(' ').enumerate() {
            if word.len() >= search_word.len() && word[..search_word.len()] == search_word {
                return i as i32 + 1;
            }
        }
        -1
    }
}
