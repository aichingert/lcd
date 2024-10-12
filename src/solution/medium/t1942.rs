use crate::Solution;

use std::cmp::Ordering;
use std::collections::BTreeSet;

#[derive(Debug)]
struct Intv {
    start: i32,
    end: i32,
    friend: usize,
}

impl Ord for Intv {
    fn cmp(&self, other: &Self) -> Ordering {
        self.start.cmp(&other.start)
    }
}

impl PartialOrd for Intv {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Intv {
    fn eq(&self, other: &Self) -> bool {
        self.start == other.start
    }
}

impl Eq for Intv {}

impl Solution {
    pub fn smallest_chair(times: Vec<Vec<i32>>, target_friend: i32) -> i32 {
        let mut m = BTreeSet::new();

        for (i, time) in times.into_iter().enumerate() {
            m.insert(Intv {
                start: time[0],
                end: time[1],
                friend: i,
            });
        }

        let f = m.pop_first().unwrap();

        if f.friend == target_friend as usize {
            return 0;
        }

        let mut ls = vec![f];

        while let Some(c) = m.pop_first() {
            let mut idx = None::<usize>;

            for (i, l) in ls.iter().enumerate() {
                if l.end <= c.start {
                    idx = Some(i);
                    break;
                }
            }

            if c.friend as i32 == target_friend {
                return idx.unwrap_or(ls.len()) as i32;
            }

            if let Some(i) = idx {
                ls[i] = c;
            } else {
                ls.push(c);
            }
        }

        panic!()
    }
}
