use crate::Solution;

use std::collections::BinaryHeap;

impl Solution {
    pub fn longest_diverse_string(a: i32, b: i32, c: i32) -> String {
        let mut ans = String::new();
        let mut q = BinaryHeap::new();
        if a > 0 {
            q.push((a, 'a'));
        }
        if b > 0 {
            q.push((b, 'b'));
        }
        if c > 0 {
            q.push((c, 'c'));
        }

        while let Some(r) = q.pop() {
            if ans.chars().rev().take(2).filter(|c| *c == r.1).count() == 2 {
                let Some(o) = q.pop() else {
                    return ans;
                };

                ans.push(o.1);

                if o.0 > 1 {
                    q.push((o.0 - 1, o.1));
                }
                q.push(r);
            } else {
                ans.push(r.1);

                if r.0 > 1 {
                    q.push((r.0 - 1, r.1));
                }
            }
        }

        ans
    }
}
