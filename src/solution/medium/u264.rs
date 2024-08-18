use std::collections::{HashSet, BinaryHeap};
use std::cmp::Reverse;

impl Solution {
    pub fn nth_ugly_number(mut n: i32) -> i32 {
        let mut s = HashSet::new();
        let mut b = BinaryHeap::from([Reverse(1u64)]);
        let mut prv = 0u64;

        while n > 0 {
            let Some(Reverse(nxt)) = b.pop() else {
                return -1;
            };
            prv = nxt;

            if s.insert(nxt * 2) {
                b.push(Reverse(nxt * 2));
            }
            if s.insert(nxt * 3) {
                b.push(Reverse(nxt * 3));
            }
            if s.insert(nxt * 5) {
                b.push(Reverse(nxt * 5));
            }
            n -= 1;
        }

        prv as i32
    }
}
