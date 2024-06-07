use crate::Solution;

// TODO: not an optimal solution
impl Solution {
    pub fn replace_words(dictionary: Vec<String>, sentence: String) -> String {
        sentence
            .split(' ')
            .map(|w| {
                let mut cur = w.to_string();

                for d in &dictionary {
                    if w.starts_with(d) && cur.len() > d.len() {
                        cur = d.to_string();
                    }
                }

                cur
            })
            .collect::<Vec<_>>()
            .join(" ")
    }
}
