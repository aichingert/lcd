use crate::Solution;

impl Solution {
    pub fn make_fancy_string(s: String) -> String {
        let mut a = Vec::new();

        for c in s.chars() {
            if a.len() > 1 && a[a.len() - 1] == c && a[a.len() - 2] == c {
                continue;
            }

            a.push(c);
        }

        a.into_iter().collect::<String>()
    }
}
