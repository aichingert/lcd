/* Leetcode daily (lcd)
 * *******************************
 * library for functions that will 
 * be used in more than one day
 */

pub fn is_palindrome(s: &[char]) -> bool  {
    for i in 0..s.len() / 2 {
        if s[i] != s[s.len() - i - 1] {
            return false;
        }
    }

    true
}
