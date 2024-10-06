use crate::Solution;

impl Solution {
    pub fn are_sentences_similar(sentence1: String, sentence2: String) -> bool {
        let mut w1 = sentence1.split(' ').collect::<Vec<_>>();
        let mut w2 = sentence2.split(' ').collect::<Vec<_>>();

        if w1.len() > w2.len() {
            (w2, w1) = (w1, w2);
        }

        let (mut s, mut e1, mut e2) = (0, w1.len() as i32 - 1, w2.len() as i32 - 1);

        while s < w1.len() && s < w2.len() && w1[s] == w2[s] {
            s += 1;
        }

        while e1 >= 0 && e2 >= 0 && w1[e1 as usize] == w2[e2 as usize] {
            e1 -= 1;
            e2 -= 1;
        }

        e1 < s as i32
    }
}
