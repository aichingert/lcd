use crate::Solution;

use std::collections::HashMap;

// TODO: this can probably be solved better
impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        if s1.len() > s2.len() {
            return false;
        }

        let chk = s1.chars().fold(HashMap::new(), |mut acc, c| {
            acc.entry(c).and_modify(|n| *n += 1).or_insert(1i32);
            acc
        });
        let chs = s2.chars().collect::<Vec<_>>();
        let mut cur = HashMap::new();

        let mut p1 = 0;
        let mut p2 = s1.len();

        for &ch in chs.iter().take(p2) {
            cur.entry(ch).and_modify(|n| *n += 1).or_insert(1i32);
        }

        for _ in s1.len()..chs.len() {
            if cur == chk {
                return true;
            }

            if *cur.get(&chs[p1]).unwrap() == 1 {
                cur.remove(&chs[p1]);
            } else {
                *cur.get_mut(&chs[p1]).unwrap() -= 1;
            }
            cur.entry(chs[p2]).and_modify(|n| *n += 1).or_insert(1i32);

            p1 += 1;
            p2 += 1;
        }

        cur == chk
    }
}
