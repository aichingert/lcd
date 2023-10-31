pub mod longest_palindromic_substring;
pub mod find_the_original_array_of_prefix_xor;

#[cfg(test)]
mod test {
    use crate::Solution;

    #[test]
    fn nr_2433_find_the_original_array_of_prefix_xor_ex_01() {
        assert_eq!(vec![5,7,2,3,2], Solution::find_array(vec![5,2,0,3,1]));
    }

    #[test]
    fn nr_2433_find_the_original_array_of_prefix_xor_ex_02() {
        assert_eq!(vec![13], Solution::find_array(vec![13]));
    }

    #[test]
    fn nr_0005_longest_palindromic_substring_ex_01() {
        assert_eq!(String::from("bab"), Solution::longest_palindrome(String::from("babad")));
    }

    #[test]
    fn nr_0005_longest_palindromic_substring_ex_02() {
        assert_eq!(String::from("bb"), Solution::longest_palindrome(String::from("cbbd")));
    }
}
