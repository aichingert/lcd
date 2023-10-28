/* Daily problem: 2023-10-27
 * aichingert
 */

use crate::Solution;
use lcd::is_palindrome;

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let chs = s.chars().collect::<Vec<_>>();

        for i in (0..=chs.len()).rev() {
            for w in chs.windows(i) {
                if is_palindrome(w) {
                    return w.iter().collect::<String>();
                }
            }
        }

        panic!()
    }
}
