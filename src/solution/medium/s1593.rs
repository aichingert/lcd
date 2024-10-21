use crate::Solution;

use std::collections::HashSet;

impl Solution {
    pub fn max_unique_split(s: String) -> i32 {
        let chs = s.chars().collect::<Vec<_>>();

        bk(&chs, HashSet::new())
    }
}

fn bk(ch: &[char], m: HashSet<Vec<char>>) -> i32 {
    if ch.is_empty() {
        return m.len() as i32;
    }

    let mut res = 0;

    for i in 1..=ch.len() {
        if m.contains(&ch[0..i]) {
            continue;
        }

        let mut cp = m.clone();
        cp.insert(ch.iter().take(i).copied().collect::<Vec<_>>());
        res = res.max(bk(&ch[i..], cp));
    }

    res
}
