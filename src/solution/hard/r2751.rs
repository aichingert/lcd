use crate::Solution;
use std::cmp::Ordering;

impl Solution {
    pub fn survived_robots_healths(positions: Vec<i32>, healths: Vec<i32>, directions: String) -> Vec<i32> {
        let mut ans = Vec::new();
        let mut st = Vec::<(usize, char, i32, i32)>::new();

        let mut comb = Vec::new();

        for (i, ch) in directions.chars().enumerate() {
            comb.push((i, ch, positions[i], healths[i]));
        }

        comb.sort_unstable_by(|(_, _, a,_), (_, _, b, _)| a.cmp(b));

        for mut cur in comb {
            if cur.1 == 'R' {
                st.push(cur);
                continue;
            } else if st.is_empty() {
                ans.push(cur);
                continue;
            }

            while let Some(mut r) = st.pop() {
                match cur.3.cmp(&r.3) {
                    Ordering::Less => {
                        r.3 -= 1;
                        st.push(r);
                        break;
                    }
                    Ordering::Equal => break,
                    Ordering::Greater => {
                        cur.3 -= 1;
                        if st.is_empty() {
                            ans.push(cur);
                        }
                    }
                }
            }
        }

        let mut res = st.into_iter().chain(ans).map(|(i, _, _, h)| (i, h)).collect::<Vec<_>>();
        res.sort_unstable_by(|a, b| a.0.cmp(&b.0));
        res.into_iter().map(|(_, h)| h).collect()
    }
}
