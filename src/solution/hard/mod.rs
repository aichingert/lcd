pub mod count_vowels_permutation;

#[cfg(test)]
mod test {
    use crate::Solution;

    #[test]
    fn nr_1220_count_vowel_permutation_ex_01() {
        assert_eq!(5, Solution::count_vowel_permutation(1));
    }

    #[test]
    fn nr_1220_count_vowel_permutation_ex_02() {
        assert_eq!(10, Solution::count_vowel_permutation(2));
    }

    #[test]
    fn nr_1220_count_vowel_permutation_ex_03() {
        assert_eq!(68, Solution::count_vowel_permutation(5));
    }
}
