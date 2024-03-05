use crate::Solution;

impl Solution {
    pub fn minimum_length(s: String) -> i32 {
        let v = s.chars().collect::<Vec<_>>();
        let (mut s, mut e) = (0, v.len() - 1);

        while s < e && v[s] == v[e] {
            let (mut f, mut b) = (s, e);

            while f < e && v[f] == v[e] {
                f += 1;
            }
            while b > s && v[b] == v[s] {
                b -= 1;
            }

            (s, e) = (f, b);
        }

        if s > e {
            0
        } else {
            (e - s + 1) as i32
        }
    }
}
