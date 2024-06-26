use crate::Solution;

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        s.split(' ').filter(|s| !s.is_empty()).last().unwrap().len() as i32
    }
}
