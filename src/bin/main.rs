mod solution;
use solution::Solution;

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn longest_palindromic_substring_0005_ex_01() {
        assert_eq!(String::from("bab"), Solution::longest_palindrome(String::from("babad")));
    }

    #[test]
    fn longest_palindromic_substring_0005_ex_02() {
        assert_eq!(String::from("bb"), Solution::longest_palindrome(String::from("cbbd")));
    }
}
