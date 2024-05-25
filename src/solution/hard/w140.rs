use crate::Solution;

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> Vec<String> {
        bkt(&s.chars().collect::<Vec<_>>(), String::new(), &word_dict)
    }
}

fn bkt(s: &[char], cur: String, dict: &[String]) -> Vec<String> {
    if s.is_empty() {
        return vec![cur];
    }

    let mut ans = Vec::new();

    for i in 0..s.len() {
        let ss = s[..=i].iter().collect::<String>();

        if dict.contains(&ss) {
            let mut cl = cur.clone();

            if !cl.is_empty() { cl.push(' '); }
            cl.push_str(&ss);

            for n in bkt(&s[i + 1..], cl, dict) {
                ans.push(n);
            }
        }
    }

    ans
}
