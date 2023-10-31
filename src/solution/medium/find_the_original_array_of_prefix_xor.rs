/* Daily problem: 2023-10-31
 * aichingert
 */

use crate::Solution;

impl Solution {
    pub fn find_array(pref: Vec<i32>) -> Vec<i32> {
        let mut arr = vec![pref[0]];

        for i in 1..pref.len() {
            arr.push(pref[i - 1] ^ pref[i]);
        }

        arr
    }
}
