use crate::ListNode;
use crate::Solution;

// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn spiral_matrix(m: i32, n: i32, mut head: Option<Box<ListNode>>) -> Vec<Vec<i32>> {
        let (n, m) = (n as usize - 1, m as usize - 1);
        let mut s = vec![vec![-1; n + 1]; m + 1];
        let (mut h, mut v) = (0, 0);
        let (mut r, mut c) = (0, 0);
        let mut d = 0;

        while let Some(node) = head {
            s[r][c] = node.val;

            match d {
                0 => {
                    if c == n - h {
                        d = 1;
                    }
                }
                1 => {
                    if r == m - v {
                        v += 1;
                        d = 2;
                    }
                }
                2 => {
                    if c == h {
                        h += 1;
                        d = 3;
                    }
                }
                _ => {
                    if r == v {
                        d = 0;
                    }
                }
            }

            (r, c) = mv(d, r, c);
            head = node.next;
        }

        s
    }
}

fn mv(d: i32, r: usize, c: usize) -> (usize, usize) {
    match d {
        0 => (r, c + 1),
        1 => (r + 1, c),
        2 => (r, c - 1),
        _ => (r - 1, c),
    }
}
