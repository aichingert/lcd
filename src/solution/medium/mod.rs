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
pub mod m1750;
pub mod c791;
pub mod b930;
pub mod p238;
pub mod c525;
pub mod i57;
pub mod m452;
pub mod t621;
pub mod f287;
pub mod f442;
pub mod s713;
pub mod l2958;
pub mod c2962;
pub mod w79;
pub mod m1249;
pub mod v678;
pub mod r950;
pub mod s129;
pub mod n200;
pub mod f1992;
pub mod o752;
pub mod m310;
pub mod l2370;
pub mod m2997;
pub mod c165;
pub mod b881;
pub mod m3075;
pub mod k786;
pub mod s861;
pub mod p1219;
pub mod s78;
pub mod p131;
pub mod t2597;
pub mod g1208;
pub mod n1404;
pub mod c1442;
pub mod s260;

#[cfg(test)]
mod test {
    use std::rc::Rc;
    use std::cell::RefCell;

    use crate::TreeNode;
    use crate::Solution;

    #[test]
    fn nr_260_ex_01() {
        assert_eq!(vec![5,3], Solution::single_number(vec![1,2,1,3,2,5]));
    }

    #[test]
    fn nr_260_ex_02() {
        assert_eq!(vec![0,-1], Solution::single_number(vec![-1,0]));
    }

    #[test]
    fn nr_260_ex_03() {
        assert_eq!(vec![0,1], Solution::single_number(vec![0,1]));
    }

    #[test]
    fn nr_1442_ex_01() {
        assert_eq!(4, Solution::count_triplets(vec![2,3,1,6,7]));
    }

    #[test]
    fn nr_1442_ex_02() {
        assert_eq!(10, Solution::count_triplets(vec![1,1,1,1,1]));
    }

    #[test]
    fn nr_1404_ex_01() {
        assert_eq!(6, Solution::num_steps("1101".to_string()));
    }

    #[test]
    fn nr_1404_ex_02() {
        assert_eq!(1, Solution::num_steps("10".to_string()));
    }

    #[test]
    fn nr_1404_ex_03() {
        assert_eq!(0, Solution::num_steps("1".to_string()));
    }

    #[test]
    fn nr_1208_ex_01() {
        assert_eq!(3, Solution::equal_substring("abcd".to_string(), "bcdf".to_string(), 3));
    }

    #[test]
    fn nr_1208_ex_02() {
        assert_eq!(1, Solution::equal_substring("abcd".to_string(), "cdef".to_string(), 3));
    }

    #[test]
    fn nr_1208_ex_03() {
        assert_eq!(1, Solution::equal_substring("abcd".to_string(), "acde".to_string(), 0));
    }

    #[test]
    fn nr_2597_ex_01() {
        assert_eq!(4, Solution::beautiful_subsets(vec![2,4,6], 2));
    }

    #[test]
    fn nr_2597_ex_02() {
        assert_eq!(1, Solution::beautiful_subsets(vec![1], 1));
    }

    #[test]
    fn nr_131_ex_01() {
        assert_eq!(vec![vec!["a".to_string(),"a".to_string(),"b".to_string()],vec!["aa".to_string(),"b".to_string()]], Solution::partition("aab".to_string()));
    }

    #[test]
    fn nr_131_ex_02() {
        assert_eq!(vec![vec!["a".to_string()]], Solution::partition("a".to_string()));
    }

    #[test]
    fn nr_78_ex_01() {
        let ac = Solution::subsets(vec![1,2,3]);
        for ex in vec![vec![],vec![1],vec![2],vec![1,2],vec![3],vec![1,3],vec![2,3],vec![1,2,3]] {
            assert!(ac.contains(&ex));
        }
    }

    #[test]
    fn nr_78_ex_02() {
        let ac = Solution::subsets(vec![0]);
        for ex in vec![vec![], vec![0]] {
            assert!(ac.contains(&ex));
        }
    }
    
    #[test]
    fn nr_1219_ex_01() {
        assert_eq!(24, Solution::get_maximum_gold(vec![vec![0,6,0],vec![5,8,7],vec![0,9,0]]));
    }

    #[test]
    fn nr_1219_ex_02() {
        assert_eq!(28, Solution::get_maximum_gold(vec![vec![1,0,7],vec![2,0,6],vec![3,4,5],vec![0,3,0],vec![9,0,20]]));
    }

    #[test]
    fn nr_861_ex_01() {
        assert_eq!(39, Solution::matrix_score(vec![vec![0,0,1,1],vec![1,0,1,0],vec![1,1,0,0]]));
    }

    #[test]
    fn nr_861_ex_02() {
        assert_eq!(1, Solution::matrix_score(vec![vec![0]]));
    }

    #[test]
    fn nr_786_ex_01() {
        assert_eq!(vec![2,5], Solution::kth_smallest_prime_fraction(vec![1,2,3,5], 3));
    }

    #[test]
    fn nr_786_ex_02() {
        assert_eq!(vec![1,7], Solution::kth_smallest_prime_fraction(vec![1,7], 1));
    }

    #[test]
    fn nr_3075_ex_01() {
        assert_eq!(4, Solution::maximum_happiness_sum(vec![1,2,3], 2));
    }

    #[test]
    fn nr_3075_ex_02() {
        assert_eq!(1, Solution::maximum_happiness_sum(vec![1,1,1,1], 2));
    }

    #[test]
    fn nr_3075_ex_03() {
        assert_eq!(5, Solution::maximum_happiness_sum(vec![2,3,4,5], 1));
    }

    #[test]
    fn nr_881_ex_01() {
        assert_eq!(1, Solution::num_rescue_boats(vec![1,2], 3));
    }

    #[test]
    fn nr_881_ex_02() {
        assert_eq!(3, Solution::num_rescue_boats(vec![3,2,2,1], 3));
    }

    #[test]
    fn nr_881_ex_03() {
        assert_eq!(4, Solution::num_rescue_boats(vec![3,5,3,4], 5));
    }

    #[test]
    fn nr_165_ex_01() {
        assert_eq!(0, Solution::compare_version("1.01".to_string(), "1.001".to_string()));
    }

    #[test]
    fn nr_165_ex_02() {
        assert_eq!(0, Solution::compare_version("1.0".to_string(), "1.0.0".to_string()));
    }

    #[test]
    fn nr_165_ex_03() {
        assert_eq!(-1, Solution::compare_version("0.1".to_string(), "1.1".to_string()));
    }

    #[test]
    fn nr_2997_ex_01() {
        assert_eq!(2, Solution::min_operations_1_lc_l(vec![2,1,3,4], 1));
    }

    #[test]
    fn nr_2997_ex_02() {
        assert_eq!(0, Solution::min_operations_1_lc_l(vec![2,0,2,0], 0));
    }

    #[test]
    fn nr_2370_ex_01() {
        assert_eq!(4, Solution::longest_ideal_string("acfgbd".to_string(), 2));
    }

    #[test]
    fn nr_2370_ex_02() {
        assert_eq!(4, Solution::longest_ideal_string("abcd".to_string(), 3));
    }

    #[test]
    fn nr_310_ex_01() {
        assert_eq!(vec![1i32], Solution::find_min_height_trees(4, vec![vec![1,0],vec![1,2],vec![1,3]]));
    }

    #[test]
    fn nr_310_ex_02() {
        assert_eq!(vec![3,4i32], Solution::find_min_height_trees(6, vec![vec![3,0],vec![3,1],vec![3,2],vec![3,4],vec![5,4]]));
    }

    #[test]
    fn nr_752_ex_01() {
        assert_eq!(6, Solution::open_lock(
            vec!["0201".to_string(),"0101".to_string(),"0102".to_string(),"1212".to_string(),"2002".to_string()], 
            "0202".to_string())
        );
    }

    #[test]
    fn nr_752_ex_02() {
        assert_eq!(1, Solution::open_lock(
            vec!["8888".to_string()], 
            "0009".to_string())
        );
    }

    #[test]
    fn nr_752_ex_03() {
        assert_eq!(-1, Solution::open_lock(
            vec!["8887".to_string(),"8889".to_string(),"8878".to_string(),"8898".to_string(),"8788".to_string(),"8988".to_string(),"7888".to_string(),"9888".to_string()],
            "8888".to_string())
        );
    }

    #[test]
    fn nr_1992_ex_01() {
        assert_eq!(vec![vec![0,0,0,0],vec![1,1,2,2]], Solution::find_farmland(vec![vec![1,0,0],vec![0,1,1],vec![0,1,1]]));
    }

    #[test]
    fn nr_1992_ex_02() {
        assert_eq!(vec![vec![0,0,1,1]], Solution::find_farmland(vec![vec![1,1], vec![1,1]]));
    }

    #[test]
    fn nr_1992_ex_03() {
        assert_eq!(Vec::<Vec<i32>>::new(), Solution::find_farmland(vec![vec![0i32]]));
    }

    #[test]
    fn nr_200_ex_01() {
        assert_eq!(1, Solution::num_islands(vec![
          vec!['1','1','1','1','0'],
          vec!['1','1','0','1','0'],
          vec!['1','1','0','0','0'],
          vec!['0','0','0','0','0']])
        );
    }

    #[test]
    fn nr_200_ex_02() {
        assert_eq!(3, Solution::num_islands(vec![
          vec!['1','1','0','0','0'],
          vec!['1','1','0','0','0'],
          vec!['0','0','1','0','0'],
          vec!['0','0','0','1','1']])
        );
    }

    #[test]
    fn nr_129_ex_01() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode{
                val: 2,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None,
                right: None,
            }))),
        })));

        assert_eq!(25, Solution::sum_numbers(root));
    }

    #[test]
    fn nr_129_ex_02() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 4,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 9,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 5,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 1,
                    left: None,
                    right: None,
                }))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 0,
                left: None,
                right: None,
            }))),
        })));

        assert_eq!(1026, Solution::sum_numbers(root));
    }

    #[test]
    fn nr_950_ex_01() {
        assert_eq!(vec![2,13,3,11,5,17,7], Solution::deck_revealed_increasing(vec![17,13,11,2,3,5,7]));
    }

    #[test]
    fn nr_950_ex_02() {
        assert_eq!(vec![1,1000], Solution::deck_revealed_increasing(vec![1,1000]));
    }

    #[test]
    fn nr_678_ex_01() {
        assert_eq!(true, Solution::check_valid_string("()".to_string()));
    }

    #[test]
    fn nr_678_ex_02() {
        assert_eq!(true, Solution::check_valid_string("(*)".to_string()));
    }

    #[test]
    fn nr_678_ex_03() {
        assert_eq!(true, Solution::check_valid_string("(*))".to_string()));
    }

    #[test]
    fn nr_1249_ex_01() {
        assert_eq!("lee(t(c)o)de".to_string(), Solution::min_remove_to_make_valid("lee(t(c)o)de)".to_string()));
    }

    #[test]
    fn nr_1249_ex_02() {
        assert_eq!("ab(c)d".to_string(), Solution::min_remove_to_make_valid("a)b(c)d".to_string()));
    }

    #[test]
    fn nr_1249_ex_03() {
        assert_eq!("".to_string(), Solution::min_remove_to_make_valid("))((".to_string()));
    }

    #[test]
    fn nr_79_ex_01() {
        assert_eq!(true, Solution::exist(vec![vec!['A','B','C','E'],vec!['S','F','C','S'],vec!['A','D','E','E']], "ABCCED".to_string()));
    }

    #[test]
    fn nr_79_ex_02() {
        assert_eq!(true, Solution::exist(vec![vec!['A','B','C','E'],vec!['S','F','C','S'],vec!['A','D','E','E']], "SEE".to_string()));
    }

    #[test]
    fn nr_79_ex_03() {
        assert_eq!(false, Solution::exist(vec![vec!['A','B','C','E'],vec!['S','F','C','S'],vec!['A','D','E','E']], "ABCB".to_string()));
    }

    #[test]
    fn nr_2962_ex_01() {
        assert_eq!(6, Solution::count_subarrays(vec![1,3,2,3,3], 2));
    }

    #[test]
    fn nr_2962_ex_02() {
        assert_eq!(0, Solution::count_subarrays(vec![1,4,2,1], 3));
    }

    #[test]
    fn nr_2958_ex_01() {
        assert_eq!(6, Solution::max_subarray_length(vec![1,2,3,1,2,3,1,2], 2));
    }

    #[test]
    fn nr_2958_ex_02() {
        assert_eq!(2, Solution::max_subarray_length(vec![1,2,1,2,1,2,1,2,1,2], 1));
    }
    
    #[test]
    fn nr_2958_ex_03() {
        assert_eq!(4, Solution::max_subarray_length(vec![5,5,5,5,5,5], 4));
    }

    #[test]
    fn nr_713_ex_01() {
        assert_eq!(8, Solution::num_subarray_product_less_than_k(vec![10,5,2,6], 100));
    }

    #[test]
    fn nr_713_ex_02() {
        assert_eq!(0, Solution::num_subarray_product_less_than_k(vec![1,2,3], 0));
    }

    #[test]
    fn nr_442_ex_01() {
        assert_eq!(vec![2,3], Solution::find_duplicates(vec![4,3,2,7,8,2,3,1]));
    }

    #[test]
    fn nr_442_ex_02() {
        assert_eq!(vec![1], Solution::find_duplicates(vec![1,1,2]));
    }

    #[test]
    fn nr_442_ex_03() {
        assert_eq!(Vec::<i32>::new(), Solution::find_duplicates(vec![1]));
    }

    #[test]
    fn nr_287_ex_01() {
        assert_eq!(2, Solution::find_duplicate(vec![1,3,4,2,2]));
    }

    #[test]
    fn nr_287_ex_02() {
        assert_eq!(3, Solution::find_duplicate(vec![3,1,3,4,2]));
    }

    #[test]
    fn nr_287_ex_03() {
        assert_eq!(3, Solution::find_duplicate(vec![3,3,3,3,3]));
    }

    #[test]
    fn nr_621_ex_01() {
        assert_eq!(8, Solution::least_interval(vec!['A','A','A','B','B','B'], 2));
    }

    #[test]
    fn nr_621_ex_02() {
        assert_eq!(6, Solution::least_interval(vec!['A','C','A','B','D','B'], 1));
    }

    #[test]
    fn nr_621_ex_03() {
        assert_eq!(10, Solution::least_interval(vec!['A','A','A','B','B','B'], 3));
    }

    #[test]
    fn nr_452_ex_01() {
        assert_eq!(2, Solution::find_min_arrow_shots(vec![vec![10,16],vec![2,8],vec![1,6],vec![7,12]]));
    }

    #[test]
    fn nr_452_ex_02() {
        assert_eq!(4, Solution::find_min_arrow_shots(vec![vec![1,2],vec![3,4],vec![5,6],vec![7,8]]));
    }

    #[test]
    fn nr_452_ex_03() {
        assert_eq!(2, Solution::find_min_arrow_shots(vec![vec![1,2],vec![2,3],vec![3,4],vec![4,5]]));
    }

    #[test]
    fn nr_57_ex_01() {
        assert_eq!(vec![vec![1,5], vec![6,9]], Solution::insert(vec![vec![1,3],vec![6,9]], vec![2,5]));
    }

    #[test]
    fn nr_57_ex_02() {
        assert_eq!(
            vec![vec![1,2], vec![3,10], vec![12,16]], 
            Solution::insert(vec![vec![1,2],vec![3,5],vec![6,7], vec![8,10],vec![12,16]], vec![4,8])
        );
    }

    #[test]
    fn nr_525_ex_01() {
        assert_eq!(2, Solution::find_max_length(vec![0,1]));
    }

    #[test]
    fn nr_525_ex_02() {
        assert_eq!(2, Solution::find_max_length(vec![0,1,0]));
    }

    #[test]
    fn nr_238_ex_01() {
        assert_eq!(vec![24,12,8,6], Solution::product_except_self(vec![1,2,3,4]));
    }

    #[test]
    fn nr_238_ex_02() {
        assert_eq!(vec![0,0,9,0,0], Solution::product_except_self(vec![-1,1,0,-3,3]));
    }

    #[test]
    fn nr_930_ex_01() {
        assert_eq!(4, Solution::num_subarrays_with_sum(vec![1,0,1,0,1], 2));
    }

    #[test]
    fn nr_930_ex_02() {
        assert_eq!(15, Solution::num_subarrays_with_sum(vec![0,0,0,0,0], 0));
    }

    #[test]
    fn nr_791_ex_01() {
        assert_eq!(format!("cbad"), Solution::custom_sort_string("cba".to_string(), "abcd".to_string()));
    }

    #[test]
    fn nr_791_ex_02() {
        assert_eq!(format!("bcad"), Solution::custom_sort_string("bcafg".to_string(), "abcd".to_string()));
    }

    #[test]
    fn nr_1750_ex_01() {
        assert_eq!(2, Solution::minimum_length(String::from("ca")));
    }

    #[test]
    fn nr_1750_ex_02() {
        assert_eq!(0, Solution::minimum_length(String::from("cabaabac")));
    }

    #[test]
    fn nr_1750_ex_03() {
        assert_eq!(3, Solution::minimum_length(String::from("aabccabba")));
    }

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

        assert!(Solution::is_even_odd_tree(root));
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

        assert!(!Solution::is_even_odd_tree(root));
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

        assert!(!Solution::is_even_odd_tree(root));
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
        assert!(Solution::close_strings("abc".to_string(), "bca".to_string()));
    }

    #[test]
    fn nr_1657_ex_02() {
        assert!(!Solution::close_strings("a".to_string(), "aa".to_string()));
    }

    #[test]
    fn nr_1657_ex_03() {
        assert!(Solution::close_strings("cabbba".to_string(), "abbccc".to_string()));
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
        assert_eq!(["Push","Push","Pop","Push"].iter().map(|s|s.to_string()).collect::<Vec<_>>(), 
                   Solution::build_array(vec![1,3], 3));
    }

    #[test]
    fn nr_1441_build_an_array_with_stack_operations_ex_02() {
        assert_eq!(["Push", "Push"].iter().map(|s| s.to_string()).collect::<Vec<_>>(), 
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
