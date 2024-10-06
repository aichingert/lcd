// TODO: this is not very good ;*(
use crate::Solution;

impl Solution {
    pub fn partition(s: String) -> Vec<Vec<String>> {
        part(s.chars().collect::<Vec<_>>().as_slice(), Vec::new())
    }
}

fn part(s: &[char], cur: Vec<String>) -> Vec<Vec<String>> {
    if s.is_empty() {
        return vec![cur];
    }

    let mut ans = Vec::new();

    for i in 0..s.len() {
        if !is_pal(&s[..=i]) {
            continue;
        }

        let mut cl = cur.clone();
        cl.push(s[..=i].iter().collect::<String>());
        for e in part(&s[i + 1..], cl) {
            ans.push(e);
        }
    }

    ans
}

fn is_pal(s: &[char]) -> bool {
    for i in 0..s.len() / 2 {
        if s[i] != s[s.len() - i - 1] {
            return false;
        }
    }

    true
}
