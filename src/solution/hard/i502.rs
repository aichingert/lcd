use crate::Solution;

use std::collections::BinaryHeap;

// TODO: this can be optimized to O(n) I think by replacing the sort with another pq
// but I am too lazy to do it right now
impl Solution {
    pub fn find_maximized_capital(mut k: i32, mut w: i32, profits: Vec<i32>, capital: Vec<i32>) -> i32 {
        let mut arr = profits.into_iter().zip(capital).collect::<Vec<_>>();
        arr.sort_by_key(|&(a, b)| b);

        let mut mhp = BinaryHeap::new();
        let mut i = 0;

        while i < arr.len() && k > 0 {
            if w >= arr[i].1 {
                mhp.push(arr[i].0);
            } else {
                if mhp.is_empty() {
                    break;
                }

                while k > 0 && w < arr[i].1 && !mhp.is_empty() {
                    k -= 1;
                    w += mhp.pop().unwrap();
                }
                continue;
            }

            i += 1;
        }

        while k > 0 && !mhp.is_empty(){
            k -= 1;
            w += mhp.pop().unwrap();
        }

        w
    }
}
