use crate::Solution;

impl Solution {
    pub fn can_change(start: String, target: String) -> bool {
        let s = start.chars().collect::<Vec<_>>();
        let t = target.chars().collect::<Vec<_>>();
        let (mut i, mut j) = (0, 0);
        let mut u = 0;

        while i < s.len() && j < t.len() {
            if t[j] != '_' {
                while i < s.len() && s[i] != t[j] {
                    if s[i] != '_' {
                        return false;
                    }

                    i += 1;
                    u += 1;
                }

                if t[j] == 'L' && j > i || t[j] == 'R' && i > j {
                    return false;
                }

                i += 1;
                j += 1;
            } else {
                u -= 1;
                j += 1;
            }
        }

        while i < s.len() {
            if s[i] == '_' {
                u += 1;
            }
            i += 1;
        }
        while j < t.len() {
            if t[j] == '_' {
                u -= 1;
            }
            j += 1;
        }

        u == 0
    }
}
