/* Daily problem: 2023-10-30
 * aichingert
 */

use crate::Solution;

impl Solution {
    pub fn sort_by_bits(mut arr: Vec<i32>) -> Vec<i32> {
        arr.sort_unstable_by(|a, b| a.count_ones().cmp(&b.count_ones()).then(a.cmp(b)));
        arr
    }
}
