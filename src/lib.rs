/* Leetcode daily (lcd)
 * *******************************
 * library for functions that will
 * be used in more than one day
 */

use std::cell::RefCell;
use std::rc::Rc;

pub mod solution;
pub use solution::Solution;

#[derive(Debug, PartialEq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    pub fn new(val: i32) -> Self {
        Self {
            val,
            left: None,
            right: None,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    pub fn new(val: i32) -> Self {
        Self { val, next: None }
    }
}

pub fn is_palindrome(s: &[char]) -> bool {
    for i in 0..s.len() / 2 {
        if s[i] != s[s.len() - i - 1] {
            return false;
        }
    }

    true
}

pub fn is_vowel(ch: char) -> bool {
    matches!(
        ch,
        'a' | 'A' | 'e' | 'E' | 'i' | 'I' | 'o' | 'O' | 'u' | 'U'
    )
}
