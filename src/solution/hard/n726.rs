use crate::Solution;
use std::collections::HashMap;

impl Solution {
    pub fn count_of_atoms(formula: String) -> String {
        let chs = formula.chars().collect::<Vec<_>>();
        let (scp, _) = parse_scope(&chs, 0);
        let mut ans = Vec::new();

        for (k, v) in scp.into_iter() {
            ans.push((k, v));
        }

        ans.sort_by(|a, b| a.0.cmp(&b.0));
        ans.into_iter().fold((String::new(), 0), |mut acc, cur| {
            acc.0.push_str(&cur.0);
            if cur.1 > 1 {
                acc.0.push_str(&cur.1.to_string());
            }
            acc
        }).0
    }
}

fn parse_scope(chs: &[char], mut p: usize) -> (HashMap<String, i32>, usize) {
    let mut scope = HashMap::new();

    if p >= chs.len() {
        return (scope, p);
    }

    while p < chs.len() {
        let mut cnt = 0;

        match chs[p] {
            '(' => {
                let (inner, nxt) = parse_scope(chs, p + 1);
                p = nxt;

                while p < chs.len() && chs[p].is_ascii_digit() {
                    cnt = cnt * 10 + (chs[p] as u8 - b'0') as i32;
                    p += 1;
                }

                cnt = cnt.max(1);
                for (k, v) in inner {
                    scope.entry(k).and_modify(|cur| *cur += v * cnt).or_insert(v * cnt);
                }
            }
            'A'..='Z' => {
                let mut s = chs[p].to_string();
                p += 1;

                while p < chs.len() && chs[p] as u8 >= b'a' && chs[p] as u8 <= b'z' {
                    s.push(chs[p]);
                    p += 1;
                }

                while p < chs.len() && chs[p].is_ascii_digit() {
                    cnt = cnt * 10 + (chs[p] as u8 - b'0') as i32;
                    p += 1;
                }

                cnt = cnt.max(1);
                scope.entry(s).and_modify(|n| *n += cnt).or_insert(cnt);
            }
            ')' => return (scope, p + 1),
            _ => unreachable!(),
        }
    }

    (scope, p)
}
