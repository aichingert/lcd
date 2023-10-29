pub mod count_vowels_permutation;
pub mod poor_pigs;

#[cfg(test)]
mod test {
    use crate::Solution;

    #[test]
    fn nr_0458_poor_pigs_ex_01() {
        assert_eq!(2, Solution::poor_pigs(4, 15, 15));
    }

    #[test]
    fn nr_0458_poor_pigs_ex_02() {
        assert_eq!(2, Solution::poor_pigs(4, 15, 30));
    }

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
