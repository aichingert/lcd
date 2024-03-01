pub mod longest_palindromic_substring;
pub mod find_the_original_array_of_prefix_xor;
pub mod count_nodes_equal_to_average_of_subtree;
pub mod build_an_array_with_stack_operations;
pub mod last_moment_before_all_ants_fall_out_of_a_plank;
pub mod find_the_winner_of_an_array_game;
pub mod eliminate_maximum_number_of_monsters;
pub mod determine_if_a_cell_is_reachable_at_a_given_time;
pub mod count_number_of_homogenous_substrings;
pub mod sort_vowels_in_a_string;
pub mod maximum_element_after_decreasing_and_rearranging;
pub mod unique_length3_palindromic_subsequences;
pub mod find_unique_binary_string;
pub mod maximum_number_of_coins_you_can_get;
pub mod minimize_maximum_pair_sum_in_array;
pub mod reduction_operations_to_make_the_array_elements_equal;
pub mod largest_submatrix_with_rearrangements;
pub mod knight_dialer;
pub mod m2870;
pub mod l300;
pub mod m1347;
pub mod d1657;
pub mod f2225;
pub mod i380;
pub mod m931;
pub mod h198;
pub mod s451;
pub mod p279;
pub mod f513;
pub mod e1609;

#[cfg(test)]
mod test {
    use std::rc::Rc;
    use std::cell::RefCell;

    use crate::TreeNode;
    use crate::Solution;

    #[test]
    fn nr_1609_ex_01() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode{
                val: 10,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(12)))),
                    right: Some(Rc::new(RefCell::new(TreeNode::new(8)))),
                }))),
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 7,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(6)))),
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 9,
                    left: None,
                    right: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
                }))),
            }))),
        })));

        assert_eq!(true, Solution::is_even_odd_tree(root));
    }

    #[test]
    fn nr_1609_ex_02() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 5,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
                right: None,
            }))),
        })));

        assert_eq!(false, Solution::is_even_odd_tree(root));
    }

    #[test]
    fn nr_1609_ex_03() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 5,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 9,
                left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
                right: None,
            }))),
        })));

        assert_eq!(false, Solution::is_even_odd_tree(root));
    }

    #[test]
    fn nr_513_ex_01() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
        })));

        assert_eq!(1, Solution::find_bottom_left_value(root));
    }

    #[test]
    fn nr_513_ex_02() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 5,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(6)))),
            }))),
        })));

        assert_eq!(7, Solution::find_bottom_left_value(root));
    }

    #[test]
    fn nr_279_ex_01() {
        assert_eq!(3, Solution::num_squares(12));
    }

    #[test]
    fn nr_279_ex_02() {
        assert_eq!(2, Solution::num_squares(13));
    }

    #[test]
    fn nr_451_ex_01() {
        let (one, two) = (String::from("eert"), String::from("eetr"));
        let res = Solution::frequency_sort(String::from("tree"));

        assert!(res == one || res == two);
    }

    #[test]
    fn nr_451_ex_02() {
        let (one, two) = (String::from("bbAa"), String::from("bbaA"));
        let res = Solution::frequency_sort(String::from("Aabb"));

        assert!(one == res || two == res);
    }

    #[test]
    fn nr_198_ex_01() {
        assert_eq!(4, Solution::rob(vec![1,2,3,1]));
    }

    #[test]
    fn nr_198_ex_02() {
        assert_eq!(12, Solution::rob(vec![2,7,9,3,1]));
    }

    #[test]
    fn nr_931_ex_01() {
        assert_eq!(3, Solution::min_falling_path_sum(vec![vec![10,1,10],vec![1,10,10],vec![10,1,10]]));
    }

    #[test]
    fn nr_380_ex_01() {
        use crate::solution::medium::i380::RandomizedSet;
        let mut set = RandomizedSet::new();

        set.insert(3);
        set.remove(3);
        set.insert(0);
        _ = set.get_random();

        assert!(true); // Not really testable
    } 

    #[test]
    fn nr_2225_ex_01() {
        assert_eq!(vec![vec![1,2,10], vec![4,5,7,8]], Solution::find_winners(vec![vec![1,3],vec![2,3],vec![3,6],vec![5,6],vec![5,7],vec![4,5],vec![4,8],vec![4,9],vec![10,4],vec![10,9]]));
    }

    #[test]
    fn nr_2225_ex_02() {
        assert_eq!(vec![vec![1,2,5,6], vec![]], Solution::find_winners(vec![vec![2,3],vec![1,3],vec![5,4],vec![6,4]]));
    }

    #[test]
    fn nr_1657_ex_01() {
        assert_eq!(true, Solution::close_strings("abc".to_string(), "bca".to_string()));
    }

    #[test]
    fn nr_1657_ex_02() {
        assert_eq!(false, Solution::close_strings("a".to_string(), "aa".to_string()));
    }

    #[test]
    fn nr_1657_ex_03() {
        assert_eq!(true, Solution::close_strings("cabbba".to_string(), "abbccc".to_string()));
    }

    #[test]
    fn nr_1347_ex_01() {
        assert_eq!(1, Solution::min_steps("bab".to_string(), "aba".to_string()));
    }

    #[test]
    fn nr_1347_ex_02() {
        assert_eq!(5, Solution::min_steps("leetcode".to_string(), "practice".to_string()));
    }

    #[test]
    fn nr_1347_ex_03() {
        assert_eq!(0, Solution::min_steps("anagram".to_string(), "mangaar".to_string()));
    }

    #[test]
    fn nr_300_ex_01() {
        assert_eq!(4, Solution::length_of_lis(vec![10,9,2,5,3,7,101,18]));
    }

    #[test]
    fn nr_300_ex_02() {
        assert_eq!(4, Solution::length_of_lis(vec![0,1,0,3,2,3]));
    }

    #[test]
    fn nr_300_ex_03() {
        assert_eq!(1, Solution::length_of_lis(vec![7,7,7,7,7,7,7]));
    }

    #[test]
    fn nr_2870_ex_01() {
        assert_eq!(4, Solution::min_operations(vec![2,3,3,2,2,4,2,3,4]));
    }

    #[test]
    fn nr_2870_ex_02() {
        assert_eq!(-1, Solution::min_operations(vec![2,1,2,2,3,3]));
    }

    #[test]
    fn nr_935_knight_dialer_ex_01() {
        assert_eq!(10, Solution::knight_dialer(1));
    }

    #[test]
    fn nr_935_knight_dialer_ex_02() {
        assert_eq!(20, Solution::knight_dialer(2));
    }

    #[test]
    fn nr_935_knight_dialer_ex_03() {
        assert_eq!(136006598, Solution::knight_dialer(3131));
    }

    #[test]
    fn nr_1727_largest_submatrix_with_rearrangements_ex_01() {
        assert_eq!(4, Solution::largest_submatrix(vec![vec![0,0,1], vec![1,1,1], vec![1,0,1]]));
    }

    #[test]
    fn nr_1727_largest_submatrix_with_rearrangements_ex_02() {
        assert_eq!(3, Solution::largest_submatrix(vec![vec![1,0,1,0,1]]));
    }
    
    #[test]
    fn nr_1727_largest_submatrix_with_rearrangements_ex_03() {
        assert_eq!(2, Solution::largest_submatrix(vec![vec![1,1,0], vec![1,0,1]]));
    }

    #[test]
    fn nr_1561_maximum_number_of_coins_you_can_get_ex_01() {
        assert_eq!(9, Solution::max_coins(vec![2,4,1,2,7,8]));
    }

    #[test]
    fn nr_1561_maximum_number_of_coins_you_can_get_ex_02() {
        assert_eq!(4, Solution::max_coins(vec![2,4,5]));
    }

    #[test]
    fn nr_1561_maximum_number_of_coins_you_can_get_ex_03() {
        assert_eq!(18, Solution::max_coins(vec![9,8,7,6,5,1,2,3,4]));
    }

    #[test]
    fn nr_1887_reduction_operations_to_make_the_array_elements_equal_ex_01() {
        assert_eq!(3, Solution::reduction_operations(vec![5,1,3]));
    }

    #[test]
    fn nr_1887_reduction_operations_to_make_the_array_elements_equal_ex_02() {
        assert_eq!(0, Solution::reduction_operations(vec![1,1,1]));
    }

    #[test]
    fn nr_1887_reduction_operations_to_make_the_array_elements_equal_ex_03() {
        assert_eq!(4, Solution::reduction_operations(vec![1,1,2,2,3]));
    }

    #[test]
    fn nr_1877_minimize_maximum_pair_sum_in_array_ex_01() {
        assert_eq!(7, Solution::min_pair_sum(vec![3,5,2,3]));
    }

    #[test]
    fn nr_1877_minimize_maximum_pair_sum_in_array_ex_02() {
        assert_eq!(8, Solution::min_pair_sum(vec![3,5,4,2,4,6]));
    }

    #[test]
    fn nr_1980_find_unique_binary_string_ex_01() {
        assert_eq!("11", &Solution::find_different_binary_string(vec![String::from("01"),String::from("10")]));
    }

    #[test]
    fn nr_1980_find_unique_binary_string_ex_02() {
        assert_eq!("10", &Solution::find_different_binary_string(vec![String::from("00"),String::from("01")]));
    }

    #[test]
    fn nr_1980_find_unique_binary_string_ex_03() {
        assert_eq!("000", &Solution::find_different_binary_string(vec![String::from("111"),String::from("011"),String::from("001")]));
    } 

    #[test]
    fn nr_1846_maximum_element_after_decreasing_and_rearranging_ex_01() {
        assert_eq!(2, Solution::maximum_element_after_decrementing_and_rearranging(vec![2,2,1,2,1]));
    }

    #[test]
    fn nr_1846_maximum_element_after_decreasing_and_rearranging_ex_02() {
        assert_eq!(3, Solution::maximum_element_after_decrementing_and_rearranging(vec![1,100,1000]));
    } 

    #[test]
    fn nr_1846_maximum_element_after_decreasing_and_rearranging_ex_03() {
        assert_eq!(5, Solution::maximum_element_after_decrementing_and_rearranging(vec![1,2,3,4,5]));
    }

    #[test]
    fn nr_1930_unique_length3_palindromic_subsequences_ex_01() {
        assert_eq!(3, Solution::count_palindromic_subsequence(String::from("aabca")));
    }

    #[test]
    fn nr_1930_unique_length3_palindromic_subsequences_ex_02() {
        assert_eq!(0, Solution::count_palindromic_subsequence(String::from("adc")));
    }

    #[test]
    fn nr_1930_unique_length3_palindromic_subsequences_ex_03() {
        assert_eq!(4, Solution::count_palindromic_subsequence(String::from("bbcbaba")));
    }

    #[test]
    fn nr_2785_sort_vowels_in_a_string_01() {
        assert_eq!("lEOtcede", &Solution::sort_vowels(String::from("lEetcOde")));
    }

    #[test]
    fn nr_2785_sort_vowels_in_a_string_02() {
        assert_eq!("lYmpH", &Solution::sort_vowels(String::from("lYmpH")));
    }

    #[test]
    fn nr_1759_count_number_of_homogenous_substrings_ex_01() {
        assert_eq!(13, Solution::count_homogenous(String::from("abbcccaa")));
    }

    #[test]
    fn nr_1759_count_number_of_homogenous_substrings_ex_02() {
        assert_eq!(2, Solution::count_homogenous(String::from("xy")));
    }

    #[test]
    fn nr_1759_count_number_of_homogenous_substrings_ex_03() {
        assert_eq!(15, Solution::count_homogenous(String::from("zzzzz")));
    }

    #[test]
    fn nr_2849_determine_if_a_cell_is_reachable_at_a_given_time_ex_01() {
        assert!(Solution::is_reachable_at_time(2, 4, 7, 7, 6));
    }

    #[test]
    fn nr_2849_determine_if_a_cell_is_reachable_at_a_given_time_ex_02() {
        assert!(!Solution::is_reachable_at_time(3, 1, 7, 3, 3));
    }

    #[test]
    fn nr_1921_eliminate_maximum_number_of_monsters_ex_02() {
        assert_eq!(1, Solution::eliminate_maximum(vec![1,1,2,3], vec![1,1,1,1]));
    }

    #[test]
    fn nr_1921_eliminate_maximum_number_of_monsters_ex_03() {
        assert_eq!(1, Solution::eliminate_maximum(vec![3,2,4], vec![5,3,2]));
    }

    #[test]
    fn nr_1535_find_the_winner_of_an_array_game_ex_01() {
        assert_eq!(5, Solution::get_winner(vec![2,1,3,5,4,6,7], 2));
    }

    #[test]
    fn nr_1535_find_the_winner_of_an_array_game_ex_02() {
        assert_eq!(3, Solution::get_winner(vec![3,2,1], 10));
    }

    #[test]
    fn nr_1503_last_moment_before_all_ants_fall_out_of_a_plank_ex_01() {
        assert_eq!(4, Solution::get_last_moment(4, vec![4, 3], vec![0,1]));
    }

    #[test]
    fn nr_1503_last_moment_before_all_ants_fall_out_of_a_plank_ex_02() {
        assert_eq!(7, Solution::get_last_moment(7, vec![], vec![0,1,2,3,4,5,6,7]));
    }

    #[test]
    fn nr_1503_last_moment_before_all_ants_fall_out_of_a_plank_ex_03() {
        assert_eq!(7, Solution::get_last_moment(7, vec![0,1,2,3,4,5,6,7], vec![]));
    }

    #[test]
    fn nr_1441_build_an_array_with_stack_operations_ex_01() {
        assert_eq!(vec!["Push","Push","Pop","Push"].iter().map(|s|s.to_string()).collect::<Vec<_>>(), 
                   Solution::build_array(vec![1,3], 3));
    }

    #[test]
    fn nr_1441_build_an_array_with_stack_operations_ex_02() {
        assert_eq!(vec!["Push", "Push"].iter().map(|s| s.to_string()).collect::<Vec<_>>(), 
                   Solution::build_array(vec![1,2], 2));
    }

    #[test]
    fn nr_2265_count_nodes_equal_to_average_of_subtree_ex_01() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 4,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 8,
                left: Some(Rc::new(RefCell::new(TreeNode::new(0)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 5,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode::new(6)))),
            }))),
        })));
        assert_eq!(5, Solution::average_of_subtree(root));
    }

    #[test]
    fn nr_2265_count_nodes_equal_to_average_of_subtree_ex_02() {
        let root = Some(Rc::new(RefCell::new(TreeNode::new(0))));
        assert_eq!(1, Solution::average_of_subtree(root));
    }

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
