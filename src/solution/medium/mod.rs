pub mod a1701;
pub mod a2192;
pub mod a2486;
pub mod b1382;
pub mod b881;
pub mod b930;
pub mod build_an_array_with_stack_operations;
pub mod c1248;
pub mod c1442;
pub mod c1497;
pub mod c165;
pub mod c1905;
pub mod c2044;
pub mod c2962;
pub mod c40;
pub mod c523;
pub mod c525;
pub mod c791;
pub mod count_nodes_equal_to_average_of_subtree;
pub mod count_number_of_homogenous_substrings;
pub mod d1110;
pub mod d1381;
pub mod d1657;
pub mod d2406;
pub mod determine_if_a_cell_is_reachable_at_a_given_time;
pub mod e1609;
pub mod eliminate_maximum_number_of_monsters;
pub mod f1371;
pub mod f1545;
pub mod f1605;
pub mod f1823;
pub mod f1894;
pub mod f1992;
pub mod f2028;
pub mod f2225;
pub mod f287;
pub mod f442;
pub mod f513;
pub mod f592;
pub mod find_the_original_array_of_prefix_xor;
pub mod find_the_winner_of_an_array_game;
pub mod find_unique_binary_string;
pub mod g1052;
pub mod g1208;
pub mod h198;
pub mod h213;
pub mod h846;
pub mod i380;
pub mod i57;
pub mod k2583;
pub mod k650;
pub mod k786;
pub mod knight_dialer;
pub mod l1405;
pub mod l1438;
pub mod l179;
pub mod l2370;
pub mod l2419;
pub mod l2958;
pub mod l300;
pub mod l386;
pub mod largest_submatrix_with_rearrangements;
pub mod last_moment_before_all_ants_fall_out_of_a_plank;
pub mod longest_palindromic_substring;
pub mod m1249;
pub mod m1347;
pub mod m1482;
pub mod m1509;
pub mod m1552;
pub mod m1717;
pub mod m1750;
pub mod m1963;
pub mod m2134;
pub mod m2285;
pub mod m2530;
pub mod m2870;
pub mod m2997;
pub mod m3016;
pub mod m3075;
pub mod m310;
pub mod m452;
pub mod m624;
pub mod m670;
pub mod m729;
pub mod m731;
pub mod m826;
pub mod m840;
pub mod m921;
pub mod m931;
pub mod m945;
pub mod m947;
pub mod m962;
pub mod maximum_element_after_decreasing_and_rearranging;
pub mod maximum_number_of_coins_you_can_get;
pub mod minimize_maximum_pair_sum_in_array;
pub mod n1404;
pub mod n1530;
pub mod n200;
pub mod o752;
pub mod p1219;
pub mod p131;
pub mod p1514;
pub mod p238;
pub mod p279;
pub mod p567;
pub mod r1190;
pub mod r1508;
pub mod r648;
pub mod r950;
pub mod r959;
pub mod reduction_operations_to_make_the_array_elements_equal;
pub mod s129;
pub mod s1593;
pub mod s1813;
pub mod s2096;
pub mod s2191;
pub mod s2326;
pub mod s260;
pub mod s2938;
pub mod s451;
pub mod s633;
pub mod s713;
pub mod s75;
pub mod s78;
pub mod s861;
pub mod s885;
pub mod s912;
pub mod s974;
pub mod sort_vowels_in_a_string;
pub mod t1942;
pub mod t2597;
pub mod t621;
pub mod unique_length3_palindromic_subsequences;
pub mod v678;
pub mod w79;
pub mod w874;
pub mod x1310;

#[cfg(test)]
mod test {
    use std::cell::RefCell;
    use std::rc::Rc;

    use crate::ListNode;
    use crate::Solution;
    use crate::TreeNode;

    #[test]
    fn nr_2583_ex_01() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 5,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 8,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
                    right: Some(Rc::new(RefCell::new(TreeNode::new(6)))),
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 9,
                left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
            }))),
        })));

        assert_eq!(13, Solution::kth_largest_level_sum(root, 2));
    }

    #[test]
    fn nr_2583_ex_02() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
                right: None,
            }))),
            right: None,
        })));

        assert_eq!(3, Solution::kth_largest_level_sum(root, 1));
    }

    #[test]
    fn nr_1593_ex_01() {
        assert_eq!(5, Solution::max_unique_split("ababccc".to_string()));
    }

    #[test]
    fn nr_1593_ex_02() {
        assert_eq!(2, Solution::max_unique_split("aba".to_string()));
    }

    #[test]
    fn nr_1593_ex_03() {
        assert_eq!(1, Solution::max_unique_split("aa".to_string()));
    }

    #[test]
    fn nr_1545_ex_01() {
        assert_eq!('0', Solution::find_kth_bit(3, 1));
    }

    #[test]
    fn nr_1545_ex_02() {
        assert_eq!('1', Solution::find_kth_bit(4, 11));
    }

    #[test]
    fn nr_2044_ex_01() {
        assert_eq!(2, Solution::count_max_or_subsets(vec![3, 1]));
    }

    #[test]
    fn nr_2044_ex_02() {
        assert_eq!(7, Solution::count_max_or_subsets(vec![2, 2, 2]));
    }

    #[test]
    fn nr_2044_ex_03() {
        assert_eq!(6, Solution::count_max_or_subsets(vec![3, 2, 1, 5]));
    }

    #[test]
    fn nr_670_ex_01() {
        assert_eq!(7236, Solution::maximum_swap(2736));
    }

    #[test]
    fn nr_670_ex_02() {
        assert_eq!(9973, Solution::maximum_swap(9973));
    }

    #[test]
    fn nr_1405_ex_01() {
        let set = vec!["ccaccbcc".to_string(), "ccbccacc".to_string()];
        assert!(set.contains(&Solution::longest_diverse_string(1, 1, 7)));
    }

    #[test]
    fn nr_1405_ex_02() {
        assert_eq!(
            "aabaa".to_string(),
            Solution::longest_diverse_string(7, 1, 0)
        );
    }

    #[test]
    fn nr_2938_ex_01() {
        assert_eq!(1, Solution::minimum_steps("101".to_string()));
    }

    #[test]
    fn nr_2938_ex_02() {
        assert_eq!(2, Solution::minimum_steps("100".to_string()));
    }

    #[test]
    fn nr_2938_ex_03() {
        assert_eq!(0, Solution::minimum_steps("0111".to_string()));
    }

    #[test]
    fn nr_2530_ex_01() {
        assert_eq!(50, Solution::max_kelements(vec![10, 10, 10, 10, 10], 5));
    }

    #[test]
    fn nr_2530_ex_02() {
        assert_eq!(17, Solution::max_kelements(vec![1, 10, 3, 3, 3], 3));
    }

    #[test]
    fn nr_2406_ex_01() {
        assert_eq!(
            3,
            Solution::min_groups(vec![
                vec![5, 10],
                vec![6, 8],
                vec![1, 5],
                vec![2, 3],
                vec![1, 10]
            ])
        );
    }

    #[test]
    fn nr_2406_ex_02() {
        assert_eq!(
            1,
            Solution::min_groups(vec![vec![1, 3], vec![5, 6], vec![8, 10], vec![11, 13]])
        );
    }

    #[test]
    fn nr_1942_ex_01() {
        assert_eq!(
            1,
            Solution::smallest_chair(vec![vec![1, 4], vec![2, 3], vec![4, 6]], 1)
        );
    }

    #[test]
    fn nr_1942_ex_02() {
        assert_eq!(
            2,
            Solution::smallest_chair(vec![vec![3, 10], vec![1, 5], vec![2, 6]], 0)
        );
    }

    #[test]
    fn nr_962_ex_01() {
        assert_eq!(4, Solution::max_width_ramp(vec![6, 0, 8, 2, 1, 5]));
    }

    #[test]
    fn nr_962_ex_02() {
        assert_eq!(
            7,
            Solution::max_width_ramp(vec![9, 8, 1, 0, 1, 9, 4, 0, 4, 1])
        );
    }

    #[test]
    fn nr_213_ex_01() {
        assert_eq!(3, Solution::rob_lc_01(vec![2, 3, 2]));
    }

    #[test]
    fn nr_213_ex_02() {
        assert_eq!(4, Solution::rob_lc_01(vec![1, 2, 3, 1]));
    }

    #[test]
    fn nr_213_ex_03() {
        assert_eq!(3, Solution::rob_lc_01(vec![1, 2, 3]));
    }

    #[test]
    fn nr_921_ex_01() {
        assert_eq!(1, Solution::min_add_to_make_valid("())".to_string()));
    }

    #[test]
    fn nr_921_ex_02() {
        assert_eq!(3, Solution::min_add_to_make_valid("(((".to_string()));
    }

    #[test]
    fn nr_1963_ex_01() {
        assert_eq!(1, Solution::min_swaps_lc_01("][][".to_string()));
    }

    #[test]
    fn nr_1963_ex_02() {
        assert_eq!(2, Solution::min_swaps_lc_01("]]][[[".to_string()));
    }

    #[test]
    fn nr_1963_ex_03() {
        assert_eq!(0, Solution::min_swaps_lc_01("[]".to_string()));
    }

    #[test]
    fn nr_1813_ex_01() {
        assert_eq!(
            true,
            Solution::are_sentences_similar("My name is Haley".to_string(), "My Haley".to_string())
        );
    }

    #[test]
    fn nr_1813_ex_02() {
        assert_eq!(
            false,
            Solution::are_sentences_similar("of".to_string(), "A lot of words".to_string())
        );
    }

    #[test]
    fn nr_1813_ex_03() {
        assert_eq!(
            true,
            Solution::are_sentences_similar("Eating right now".to_string(), "Eating".to_string())
        );
    }

    #[test]
    fn nr_567_ex_01() {
        assert_eq!(
            true,
            Solution::check_inclusion("ab".to_string(), "eidbaooo".to_string())
        );
    }

    #[test]
    fn nr_567_ex_02() {
        assert_eq!(
            false,
            Solution::check_inclusion("ab".to_string(), "eidboaoo".to_string())
        );
    }

    #[test]
    fn nr_1497_ex_01() {
        assert_eq!(
            true,
            Solution::can_arrange(vec![1, 2, 3, 4, 5, 10, 6, 7, 8, 9], 5)
        );
    }

    #[test]
    fn nr_1497_ex_02() {
        assert_eq!(true, Solution::can_arrange(vec![1, 2, 3, 4, 5, 6], 7));
    }

    #[test]
    fn nr_1497_ex_03() {
        assert_eq!(false, Solution::can_arrange(vec![1, 2, 3, 4, 5, 6], 10));
    }

    #[test]
    fn nr_1381_ex_01() {
        let mut s = super::d1381::CustomStack::new(3);

        s.push(1);
        s.push(2);
        assert_eq!(2, s.pop());
        s.push(2);
        s.push(3);
        s.push(4);
        s.increment(5, 100);
        s.increment(2, 100);
        assert_eq!(103, s.pop());
        assert_eq!(202, s.pop());
        assert_eq!(201, s.pop());
        assert_eq!(-1, s.pop());
    }

    #[test]
    fn nr_729_ex_01() {
        let mut c = super::m729::MyCalendar::default();

        assert_eq!(true, c.book(10, 20));
        assert_eq!(false, c.book(15, 25));
        assert_eq!(true, c.book(20, 30));
    }

    #[test]
    fn nr_731_ex_01() {
        let mut c = super::m731::MyCalendarTwo::default();

        assert_eq!(true, c.book(10, 20));
        assert_eq!(true, c.book(50, 60));
        assert_eq!(true, c.book(10, 40));
        assert_eq!(false, c.book(5, 15));
        assert_eq!(true, c.book(5, 10));
        assert_eq!(true, c.book(25, 55));
    }

    #[test]
    fn nr_386_ex_01() {
        assert_eq!(
            vec![1, 10, 11, 12, 13, 2, 3, 4, 5, 6, 7, 8, 9],
            Solution::lexical_order(13)
        );
    }

    #[test]
    fn nr_386_ex_02() {
        assert_eq!(vec![1, 2], Solution::lexical_order(2));
    }

    #[test]
    fn nr_179_ex_01() {
        assert_eq!("210".to_string(), Solution::largest_number(vec![10, 2]));
    }

    #[test]
    fn nr_179_ex_02() {
        assert_eq!(
            "9534330".to_string(),
            Solution::largest_number(vec![3, 30, 34, 5, 9])
        );
    }

    #[test]
    fn nr_1371_ex_01() {
        assert_eq!(
            13,
            Solution::find_the_longest_substring("eleetminicoworoep".to_string())
        );
    }

    #[test]
    fn nr_1371_ex_02() {
        assert_eq!(
            5,
            Solution::find_the_longest_substring("leetcodeisgreate".to_string())
        );
    }

    #[test]
    fn nr_1371_ex_03() {
        assert_eq!(
            6,
            Solution::find_the_longest_substring("bcbcbc".to_string())
        );
    }

    #[test]
    fn nr_2419_ex_01() {
        assert_eq!(2, Solution::longest_subarray_lc_1(vec![1, 2, 3, 3, 2, 2]));
    }

    #[test]
    fn nr_2419_ex_02() {
        assert_eq!(1, Solution::longest_subarray_lc_1(vec![1, 2, 3, 4]));
    }

    #[test]
    fn nr_1310_ex_01() {
        assert_eq!(
            vec![2, 7, 14, 8],
            Solution::xor_queries(
                vec![1, 3, 4, 8],
                vec![vec![0, 1], vec![1, 2], vec![0, 3], vec![3, 3]]
            )
        );
    }

    #[test]
    fn nr_1310_ex_02() {
        assert_eq!(
            vec![8, 0, 4, 4],
            Solution::xor_queries(
                vec![4, 8, 2, 10],
                vec![vec![2, 3], vec![1, 3], vec![0, 0], vec![0, 3]]
            )
        );
    }

    #[test]
    fn nr_2326_ex_01() {
        let list = ListNode {
            val: 3,
            next: Some(Box::new(ListNode {
                val: 0,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode {
                        val: 6,
                        next: Some(Box::new(ListNode {
                            val: 8,
                            next: Some(Box::new(ListNode {
                                val: 1,
                                next: Some(Box::new(ListNode {
                                    val: 7,
                                    next: Some(Box::new(ListNode {
                                        val: 9,
                                        next: Some(Box::new(ListNode {
                                            val: 4,
                                            next: Some(Box::new(ListNode {
                                                val: 2,
                                                next: Some(Box::new(ListNode {
                                                    val: 5,
                                                    next: Some(Box::new(ListNode {
                                                        val: 5,
                                                        next: Some(Box::new(ListNode::new(0))),
                                                    })),
                                                })),
                                            })),
                                        })),
                                    })),
                                })),
                            })),
                        })),
                    })),
                })),
            })),
        };

        assert_eq!(
            vec![
                vec![3, 0, 2, 6, 8],
                vec![5, 0, -1, -1, 1],
                vec![5, 2, 4, 9, 7]
            ],
            Solution::spiral_matrix(3, 5, Some(Box::new(list)))
        );
    }

    #[test]
    fn nr_2326_ex_02() {
        let list = Some(Box::new(ListNode {
            val: 0,
            next: Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode::new(2))),
            })),
        }));

        assert_eq!(vec![vec![0, 1, 2, -1]], Solution::spiral_matrix(1, 4, list));
    }

    #[test]
    fn nr_2028_ex_01() {
        assert_eq!(vec![6, 6], Solution::missing_rolls(vec![3, 2, 4, 3], 4, 2));
    }

    #[test]
    fn nr_2028_ex_02() {
        assert_eq!(
            vec![6, 1, 1, 1],
            Solution::missing_rolls(vec![1, 5, 6], 3, 4)
        );
    }

    #[test]
    fn nr_2028_ex_03() {
        assert_eq!(
            Vec::<i32>::new(),
            Solution::missing_rolls(vec![1, 2, 3, 4], 6, 4)
        );
    }

    #[test]
    fn nr_874_ex_01() {
        assert_eq!(25, Solution::robot_sim(vec![4, -1, 3], vec![]));
    }

    #[test]
    fn nr_874_ex_02() {
        assert_eq!(
            65,
            Solution::robot_sim(vec![4, -1, 4, -2, 4], vec![vec![2, 4]])
        );
    }

    #[test]
    fn nr_874_ex_03() {
        assert_eq!(36, Solution::robot_sim(vec![6, -1, -1, 6], vec![]));
    }

    #[test]
    fn nr_1894_ex_01() {
        assert_eq!(0, Solution::chalk_replacer(vec![5, 1, 5], 22));
    }

    #[test]
    fn nr_1894_ex_02() {
        assert_eq!(1, Solution::chalk_replacer(vec![3, 4, 1, 2], 25));
    }

    #[test]
    fn nr_947_ex_01() {
        assert_eq!(
            5,
            Solution::remove_stones_lc_1(vec![
                vec![0, 0],
                vec![0, 1],
                vec![1, 0],
                vec![1, 2],
                vec![2, 1],
                vec![2, 2]
            ])
        );
    }

    #[test]
    fn nr_947_ex_02() {
        assert_eq!(
            3,
            Solution::remove_stones_lc_1(vec![
                vec![0, 0],
                vec![0, 2],
                vec![1, 1],
                vec![2, 0],
                vec![2, 2]
            ])
        );
    }

    #[test]
    fn nr_947_ex_03() {
        assert_eq!(0, Solution::remove_stones_lc_1(vec![vec![0, 0]]));
    }

    #[test]
    fn nr_1905_ex_01() {
        assert_eq!(
            3,
            Solution::count_sub_islands(
                vec![
                    vec![1, 1, 1, 0, 0],
                    vec![0, 1, 1, 1, 1],
                    vec![0, 0, 0, 0, 0],
                    vec![1, 0, 0, 0, 0],
                    vec![1, 1, 0, 1, 1]
                ],
                vec![
                    vec![1, 1, 1, 0, 0],
                    vec![0, 0, 1, 1, 1],
                    vec![0, 1, 0, 0, 0],
                    vec![1, 0, 1, 1, 0],
                    vec![0, 1, 0, 1, 0]
                ]
            )
        );
    }

    #[test]
    fn nr_1905_ex_02() {
        assert_eq!(
            2,
            Solution::count_sub_islands(
                vec![
                    vec![1, 0, 1, 0, 1],
                    vec![1, 1, 1, 1, 1],
                    vec![0, 0, 0, 0, 0],
                    vec![1, 1, 1, 1, 1],
                    vec![1, 0, 1, 0, 1]
                ],
                vec![
                    vec![0, 0, 0, 0, 0],
                    vec![1, 1, 1, 1, 1],
                    vec![0, 1, 0, 1, 0],
                    vec![0, 1, 0, 1, 0],
                    vec![1, 0, 0, 0, 1]
                ]
            )
        );
    }

    #[test]
    fn nr_1514_ex_01() {
        assert_eq!(
            0.25,
            Solution::max_probability(
                3,
                vec![vec![0, 1], vec![1, 2], vec![0, 2]],
                vec![0.5, 0.5, 0.2],
                0,
                2
            )
        );
    }

    #[test]
    fn nr_1514_ex_02() {
        assert_eq!(
            0.3,
            Solution::max_probability(
                3,
                vec![vec![0, 1], vec![1, 2], vec![0, 2]],
                vec![0.5, 0.5, 0.3],
                0,
                2
            )
        );
    }

    #[test]
    fn nr_1514_ex_03() {
        assert_eq!(
            0.0,
            Solution::max_probability(3, vec![vec![0, 1]], vec![0.5], 0, 2)
        );
    }

    #[test]
    fn nr_592_ex_01() {
        assert_eq!(
            "0/1".to_string(),
            Solution::fraction_addition("-1/2+1/2".to_string())
        );
    }

    #[test]
    fn nr_592_ex_02() {
        assert_eq!(
            "1/3".to_string(),
            Solution::fraction_addition("-1/2+1/2+1/3".to_string())
        );
    }

    #[test]
    fn nr_592_ex_03() {
        assert_eq!(
            "-1/6".to_string(),
            Solution::fraction_addition("1/3-1/2".to_string())
        );
    }

    #[test]
    fn nr_650_ex_01() {
        assert_eq!(3, Solution::min_steps_lc_1(3));
    }

    #[test]
    fn nr_650_ex_02() {
        assert_eq!(0, Solution::min_steps_lc_1(1));
    }

    #[test]
    fn nr_624_ex_01() {
        assert_eq!(
            4,
            Solution::max_distance_lc_1(vec![vec![1, 2, 3], vec![4, 5], vec![1, 2, 3]])
        );
    }

    #[test]
    fn nr_624_ex_02() {
        assert_eq!(0, Solution::max_distance_lc_1(vec![vec![1], vec![1]]));
    }

    #[test]
    fn nr_40_ex_01() {
        assert_eq!(
            vec![vec![1, 1, 6], vec![1, 2, 5], vec![1, 7], vec![2, 6]],
            Solution::combination_sum2(vec![10, 1, 2, 7, 6, 1, 5], 8)
        );
    }

    #[test]
    fn nr_40_ex_02() {
        assert_eq!(
            vec![vec![1, 2, 2], vec![5]],
            Solution::combination_sum2(vec![2, 5, 2, 1, 2], 5)
        );
    }

    #[test]
    fn nr_959_ex_01() {
        assert_eq!(
            2,
            Solution::regions_by_slashes(vec![" /".to_string(), "/ ".to_string()])
        );
    }

    #[test]
    fn nr_959_ex_02() {
        assert_eq!(
            1,
            Solution::regions_by_slashes(vec![" /".to_string(), "  ".to_string()])
        );
    }

    #[test]
    fn nr_959_ex_03() {
        assert_eq!(
            5,
            Solution::regions_by_slashes(vec!["/\\".to_string(), "\\/".to_string()])
        );
    }

    #[test]
    fn nr_840_ex_01() {
        assert_eq!(
            1,
            Solution::num_magic_squares_inside(vec![
                vec![4, 3, 8, 4],
                vec![9, 5, 1, 9],
                vec![2, 7, 6, 2]
            ])
        );
    }

    #[test]
    fn nr_840_ex_02() {
        assert_eq!(0, Solution::num_magic_squares_inside(vec![vec![8]]));
    }

    #[test]
    fn nr_885_ex_01() {
        assert_eq!(
            vec![vec![0, 0], vec![0, 1], vec![0, 2], vec![0, 3]],
            Solution::spiral_matrix_iii(1, 4, 0, 0)
        );
    }

    #[test]
    fn nr_885_ex_02() {
        assert_eq!(
            vec![
                vec![1, 4],
                vec![1, 5],
                vec![2, 5],
                vec![2, 4],
                vec![2, 3],
                vec![1, 3],
                vec![0, 3],
                vec![0, 4],
                vec![0, 5],
                vec![3, 5],
                vec![3, 4],
                vec![3, 3],
                vec![3, 2],
                vec![2, 2],
                vec![1, 2],
                vec![0, 2],
                vec![4, 5],
                vec![4, 4],
                vec![4, 3],
                vec![4, 2],
                vec![4, 1],
                vec![3, 1],
                vec![2, 1],
                vec![1, 1],
                vec![0, 1],
                vec![4, 0],
                vec![3, 0],
                vec![2, 0],
                vec![1, 0],
                vec![0, 0]
            ],
            Solution::spiral_matrix_iii(5, 6, 1, 4)
        );
    }

    #[test]
    fn nr_3016_ex_01() {
        assert_eq!(5, Solution::minimum_pushes("abcde".to_string()));
    }

    #[test]
    fn nr_3016_ex_02() {
        assert_eq!(12, Solution::minimum_pushes("xyzxyzxyzxyz".to_string()));
    }

    #[test]
    fn nr_3016_ex_03() {
        assert_eq!(
            24,
            Solution::minimum_pushes("aabbccddeeffgghhiiiiii".to_string())
        );
    }

    #[test]
    fn nr_1508_ex_01() {
        assert_eq!(13, Solution::range_sum(vec![1, 2, 3, 4], 4, 1, 5));
    }

    #[test]
    fn nr_1508_ex_02() {
        assert_eq!(6, Solution::range_sum(vec![1, 2, 3, 4], 4, 3, 4));
    }

    #[test]
    fn nr_1508_ex_03() {
        assert_eq!(50, Solution::range_sum(vec![1, 2, 3, 4], 4, 1, 10));
    }

    #[test]
    fn nr_2134_ex_01() {
        assert_eq!(1, Solution::min_swaps(vec![0, 1, 0, 1, 1, 0, 0]));
    }

    #[test]
    fn nr_2134_ex_02() {
        assert_eq!(2, Solution::min_swaps(vec![0, 1, 1, 1, 0, 0, 1, 1, 0]));
    }

    #[test]
    fn nr_2134_ex_03() {
        assert_eq!(0, Solution::min_swaps(vec![1, 1, 0, 0, 1]));
    }

    #[test]
    fn nr_912_ex_01() {
        assert_eq!(vec![1, 2, 3, 5], Solution::sort_array(vec![5, 2, 3, 1]));
    }

    #[test]
    fn nr_912_ex_02() {
        assert_eq!(
            vec![0, 0, 1, 1, 2, 5],
            Solution::sort_array(vec![5, 1, 1, 2, 0, 0])
        );
    }

    #[test]
    fn nr_2191_ex_01() {
        assert_eq!(
            vec![338, 38, 991],
            Solution::sort_jumbled(vec![8, 9, 4, 0, 2, 1, 3, 5, 7, 6], vec![991, 338, 38])
        );
    }

    #[test]
    fn nr_2191_ex_02() {
        assert_eq!(
            vec![123, 456, 789],
            Solution::sort_jumbled(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9], vec![789, 456, 123])
        );
    }

    #[test]
    fn nr_1605_ex_01() {
        assert_eq!(
            vec![vec![3, 0], vec![1, 7]],
            Solution::restore_matrix(vec![3, 8], vec![4, 7])
        );
    }

    #[test]
    fn nr_1605_ex_02() {
        assert_eq!(
            vec![vec![5, 0, 0], vec![3, 4, 0], vec![0, 2, 8]],
            Solution::restore_matrix(vec![5, 7, 10], vec![8, 6, 8])
        );
    }

    #[test]
    fn nr_1530_ex_01() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None,
                right: None,
            }))),
        })));

        assert_eq!(1, Solution::count_pairs(root, 3));
    }

    #[test]
    fn nr_1530_ex_02() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: Some(Rc::new(RefCell::new(TreeNode::new(6)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
            }))),
        })));

        assert_eq!(2, Solution::count_pairs(root, 3));
    }

    #[test]
    fn nr_1530_ex_03() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 7,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: Some(Rc::new(RefCell::new(TreeNode::new(6)))),
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: None,
                    right: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
                }))),
            }))),
        })));

        assert_eq!(1, Solution::count_pairs(root, 3));
    }

    #[test]
    fn nr_1110_ex_01() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: Some(Rc::new(RefCell::new(TreeNode::new(6)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
            }))),
        })));
        let expect = vec![
            Some(Rc::new(RefCell::new(TreeNode::new(6)))),
            Some(Rc::new(RefCell::new(TreeNode::new(7)))),
            Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
                    right: None,
                }))),
                right: None,
            }))),
        ];

        assert_eq!(expect, Solution::del_nodes(root, vec![3, 5]));
    }

    #[test]
    fn nr_1110_ex_02() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: None,
                right: None,
            }))),
        })));
        let expect = vec![Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: None,
                right: None,
            }))),
        })))];

        assert_eq!(expect, Solution::del_nodes(root, vec![3]));
    }

    #[test]
    fn nr_2096_ex_01() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 5,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode::new(6)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
            }))),
        })));

        assert_eq!("UURL".to_string(), Solution::get_directions(root, 3, 6));
    }

    #[test]
    fn nr_2096_ex_02() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
            right: None,
        })));

        assert_eq!("L".to_string(), Solution::get_directions(root, 2, 1));
    }

    #[test]
    fn nr_1717_ex_01() {
        assert_eq!(19, Solution::maximum_gain("cdbcbbaaabab".to_string(), 4, 5));
    }

    #[test]
    fn nr_1717_ex_02() {
        assert_eq!(
            20,
            Solution::maximum_gain("aabbaaxybbaabb".to_string(), 5, 4)
        );
    }

    #[test]
    fn nr_1190_ex_01() {
        assert_eq!(
            "dcba".to_string(),
            Solution::reverse_parentheses(String::from("(abcd)"))
        );
    }

    #[test]
    fn nr_1190_ex_02() {
        assert_eq!(
            "iloveu".to_string(),
            Solution::reverse_parentheses(String::from("(u(love)i)"))
        );
    }

    #[test]
    fn nr_1190_ex_03() {
        assert_eq!(
            "leetcode".to_string(),
            Solution::reverse_parentheses(String::from("(ed(et(oc))el)"))
        );
    }

    #[test]
    fn nr_1701_ex_01() {
        assert_eq!(
            5f64,
            Solution::average_waiting_time(vec![vec![1, 2], vec![2, 5], vec![4, 3]])
        );
    }

    #[test]
    fn nr_1701_ex_02() {
        assert_eq!(
            3.25f64,
            Solution::average_waiting_time(vec![vec![5, 2], vec![5, 4], vec![10, 3], vec![20, 1]])
        );
    }

    #[test]
    fn nr_1823_ex_01() {
        assert_eq!(3, Solution::find_the_winner(5, 2));
    }

    #[test]
    fn nr_1823_ex_02() {
        assert_eq!(1, Solution::find_the_winner(6, 5));
    }

    #[test]
    fn nr_1509_ex_01() {
        assert_eq!(0, Solution::min_difference(vec![5, 3, 2, 4]));
    }

    #[test]
    fn nr_1509_ex_02() {
        assert_eq!(1, Solution::min_difference(vec![1, 5, 0, 10, 14]));
    }

    #[test]
    fn nr_1509_ex_03() {
        assert_eq!(0, Solution::min_difference(vec![3, 100, 20]));
    }

    #[test]
    fn nr_2192_ex_01() {
        assert_eq!(
            vec![
                vec![],
                vec![],
                vec![],
                vec![0, 1],
                vec![0, 2],
                vec![0, 1, 3],
                vec![0, 1, 2, 3, 4],
                vec![0, 1, 2, 3]
            ],
            Solution::get_ancestors(
                8,
                vec![
                    vec![0, 3],
                    vec![0, 4],
                    vec![1, 3],
                    vec![2, 4],
                    vec![2, 7],
                    vec![3, 5],
                    vec![3, 6],
                    vec![3, 7],
                    vec![4, 6]
                ]
            )
        );
    }

    #[test]
    fn nr_2192_ex_02() {
        assert_eq!(
            vec![vec![], vec![0], vec![0, 1], vec![0, 1, 2], vec![0, 1, 2, 3]],
            Solution::get_ancestors(
                5,
                vec![
                    vec![0, 1],
                    vec![0, 2],
                    vec![0, 3],
                    vec![0, 4],
                    vec![1, 2],
                    vec![1, 3],
                    vec![1, 4],
                    vec![2, 3],
                    vec![2, 4],
                    vec![3, 4]
                ]
            )
        );
    }

    #[test]
    fn nr_2285_ex_01() {
        assert_eq!(
            43,
            Solution::maximum_importance(
                5,
                vec![
                    vec![0, 1],
                    vec![1, 2],
                    vec![2, 3],
                    vec![0, 2],
                    vec![1, 3],
                    vec![2, 4]
                ]
            )
        );
    }

    #[test]
    fn nr_2285_ex_02() {
        assert_eq!(
            20,
            Solution::maximum_importance(5, vec![vec![0, 3], vec![2, 4], vec![1, 3]])
        );
    }

    #[test]
    fn nr_1382_ex_01() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: None,
                    right: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
                }))),
            }))),
        })));

        let expect = Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
        })));

        assert_eq!(expect, Solution::balance_bst(root));
    }

    #[test]
    fn nr_1382_ex_02() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
        })));

        assert_eq!(root.clone(), Solution::balance_bst(root));
    }

    #[test]
    fn nr_1438_ex_01() {
        assert_eq!(2, Solution::longest_subarray(vec![8, 2, 4, 7], 4));
    }

    #[test]
    fn nr_1438_ex_02() {
        assert_eq!(4, Solution::longest_subarray(vec![10, 1, 2, 4, 7, 2], 5));
    }

    #[test]
    fn nr_1438_ex_03() {
        assert_eq!(
            3,
            Solution::longest_subarray(vec![4, 2, 2, 2, 4, 4, 2, 2], 0)
        );
    }

    #[test]
    fn nr_1248_ex_01() {
        assert_eq!(2, Solution::number_of_subarrays(vec![1, 1, 2, 1, 1], 3));
    }

    #[test]
    fn nr_1248_ex_02() {
        assert_eq!(0, Solution::number_of_subarrays(vec![2, 4, 6], 1));
    }

    #[test]
    fn nr_1248_ex_03() {
        assert_eq!(
            16,
            Solution::number_of_subarrays(vec![2, 2, 2, 1, 2, 2, 1, 2, 2, 2], 2)
        );
    }

    #[test]
    fn nr_1052_ex_01() {
        assert_eq!(
            16,
            Solution::max_satisfied(
                vec![1, 0, 1, 2, 1, 1, 7, 5],
                vec![0, 1, 0, 1, 0, 1, 0, 1],
                3
            )
        );
    }

    #[test]
    fn nr_1052_ex_02() {
        assert_eq!(1, Solution::max_satisfied(vec![1], vec![0], 1));
    }

    #[test]
    fn nr_1552_ex_01() {
        assert_eq!(3, Solution::max_distance(vec![1, 2, 3, 4, 7], 3));
    }

    #[test]
    fn nr_1552_ex_02() {
        assert_eq!(
            999999999,
            Solution::max_distance(vec![5, 4, 3, 2, 1, 1000000000], 2)
        );
    }

    #[test]
    fn nr_1482_ex_01() {
        assert_eq!(3, Solution::min_days(vec![1, 10, 3, 10, 2], 3, 1));
    }

    #[test]
    fn nr_1482_ex_02() {
        assert_eq!(-1, Solution::min_days(vec![1, 10, 3, 10, 2], 3, 2));
    }

    #[test]
    fn nr_1482_ex_03() {
        assert_eq!(12, Solution::min_days(vec![7, 7, 7, 7, 12, 7, 7], 2, 3));
    }

    #[test]
    fn nr_826_ex_01() {
        assert_eq!(
            100,
            Solution::max_profit_assignment(
                vec![2, 4, 6, 8, 10],
                vec![10, 20, 30, 40, 50],
                vec![4, 5, 6, 7]
            )
        );
    }

    #[test]
    fn nr_826_ex_02() {
        assert_eq!(
            0,
            Solution::max_profit_assignment(vec![85, 47, 57], vec![24, 66, 99], vec![40, 25, 25])
        );
    }

    #[test]
    fn nr_633_ex_01() {
        assert!(Solution::judge_square_sum(5));
    }

    #[test]
    fn nr_633_ex_02() {
        assert!(!Solution::judge_square_sum(3));
    }

    #[test]
    fn nr_945_ex_01() {
        assert_eq!(1, Solution::min_increment_for_unique(vec![1, 2, 2]));
    }

    #[test]
    fn nr_945_ex_02() {
        assert_eq!(
            6,
            Solution::min_increment_for_unique(vec![3, 2, 1, 2, 1, 7])
        );
    }

    #[test]
    fn nr_75_ex_01() {
        let mut vec = vec![2, 0, 2, 1, 1, 0];
        Solution::sort_colors(&mut vec);
        assert_eq!(vec![0, 0, 1, 1, 2, 2], vec);
    }

    #[test]
    fn nr_75_ex_02() {
        let mut vec = vec![0, 1, 2];
        Solution::sort_colors(&mut vec);
        assert_eq!(vec![0, 1, 2], vec);
    }

    #[test]
    fn nr_974_ex_01() {
        assert_eq!(7, Solution::subarrays_div_by_k(vec![4, 5, 0, -2, -3, 1], 5));
    }

    #[test]
    fn nr_974_ex_02() {
        assert_eq!(0, Solution::subarrays_div_by_k(vec![5], 9));
    }

    #[test]
    fn nr_523_ex_01() {
        assert!(Solution::check_subarray_sum(vec![23, 2, 4, 6, 7], 6));
    }

    #[test]
    fn nr_523_ex_02() {
        assert!(Solution::check_subarray_sum(vec![23, 2, 6, 4, 7], 6));
    }

    #[test]
    fn nr_523_ex_03() {
        assert!(!Solution::check_subarray_sum(vec![23, 2, 6, 4, 7], 13));
    }

    #[test]
    fn nr_648_ex_01() {
        assert_eq!(
            "the cat was rat by the bat".to_string(),
            Solution::replace_words(
                vec!["cat".to_string(), "bat".to_string(), "rat".to_string()],
                "the cattle was rattled by the battery".to_string()
            )
        );
    }

    #[test]
    fn nr_648_ex_02() {
        assert_eq!(
            "a a b c".to_string(),
            Solution::replace_words(
                vec!["a".to_string(), "b".to_string(), "c".to_string()],
                "aadsfasf absbs bbab cadsfafs".to_string()
            )
        );
    }

    #[test]
    fn nr_846_ex_01() {
        assert!(Solution::is_n_straight_hand(
            vec![1, 2, 3, 6, 2, 3, 4, 7, 8],
            3
        ));
    }

    #[test]
    fn nr_846_ex_02() {
        assert!(!Solution::is_n_straight_hand(vec![1, 2, 3, 4, 5], 4));
    }

    #[test]
    fn nr_2486_ex_01() {
        assert_eq!(
            4,
            Solution::append_characters("coaching".to_string(), "coding".to_string())
        );
    }

    #[test]
    fn nr_2486_ex_02() {
        assert_eq!(
            0,
            Solution::append_characters("abcde".to_string(), "a".to_string())
        );
    }

    #[test]
    fn nr_2486_ex_03() {
        assert_eq!(
            5,
            Solution::append_characters("z".to_string(), "abcde".to_string())
        );
    }

    #[test]
    fn nr_260_ex_01() {
        assert_eq!(vec![5, 3], Solution::single_number(vec![1, 2, 1, 3, 2, 5]));
    }

    #[test]
    fn nr_260_ex_02() {
        assert_eq!(vec![0, -1], Solution::single_number(vec![-1, 0]));
    }

    #[test]
    fn nr_260_ex_03() {
        assert_eq!(vec![0, 1], Solution::single_number(vec![0, 1]));
    }

    #[test]
    fn nr_1442_ex_01() {
        assert_eq!(4, Solution::count_triplets(vec![2, 3, 1, 6, 7]));
    }

    #[test]
    fn nr_1442_ex_02() {
        assert_eq!(10, Solution::count_triplets(vec![1, 1, 1, 1, 1]));
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
        assert_eq!(
            3,
            Solution::equal_substring("abcd".to_string(), "bcdf".to_string(), 3)
        );
    }

    #[test]
    fn nr_1208_ex_02() {
        assert_eq!(
            1,
            Solution::equal_substring("abcd".to_string(), "cdef".to_string(), 3)
        );
    }

    #[test]
    fn nr_1208_ex_03() {
        assert_eq!(
            1,
            Solution::equal_substring("abcd".to_string(), "acde".to_string(), 0)
        );
    }

    #[test]
    fn nr_2597_ex_01() {
        assert_eq!(4, Solution::beautiful_subsets(vec![2, 4, 6], 2));
    }

    #[test]
    fn nr_2597_ex_02() {
        assert_eq!(1, Solution::beautiful_subsets(vec![1], 1));
    }

    #[test]
    fn nr_131_ex_01() {
        assert_eq!(
            vec![
                vec!["a".to_string(), "a".to_string(), "b".to_string()],
                vec!["aa".to_string(), "b".to_string()]
            ],
            Solution::partition("aab".to_string())
        );
    }

    #[test]
    fn nr_131_ex_02() {
        assert_eq!(
            vec![vec!["a".to_string()]],
            Solution::partition("a".to_string())
        );
    }

    #[test]
    fn nr_78_ex_01() {
        let ac = Solution::subsets(vec![1, 2, 3]);
        for ex in [
            vec![],
            vec![1],
            vec![2],
            vec![1, 2],
            vec![3],
            vec![1, 3],
            vec![2, 3],
            vec![1, 2, 3],
        ] {
            assert!(ac.contains(&ex));
        }
    }

    #[test]
    fn nr_78_ex_02() {
        let ac = Solution::subsets(vec![0]);
        for ex in [vec![], vec![0]] {
            assert!(ac.contains(&ex));
        }
    }

    #[test]
    fn nr_1219_ex_01() {
        assert_eq!(
            24,
            Solution::get_maximum_gold(vec![vec![0, 6, 0], vec![5, 8, 7], vec![0, 9, 0]])
        );
    }

    #[test]
    fn nr_1219_ex_02() {
        assert_eq!(
            28,
            Solution::get_maximum_gold(vec![
                vec![1, 0, 7],
                vec![2, 0, 6],
                vec![3, 4, 5],
                vec![0, 3, 0],
                vec![9, 0, 20]
            ])
        );
    }

    #[test]
    fn nr_861_ex_01() {
        assert_eq!(
            39,
            Solution::matrix_score(vec![vec![0, 0, 1, 1], vec![1, 0, 1, 0], vec![1, 1, 0, 0]])
        );
    }

    #[test]
    fn nr_861_ex_02() {
        assert_eq!(1, Solution::matrix_score(vec![vec![0]]));
    }

    #[test]
    fn nr_786_ex_01() {
        assert_eq!(
            vec![2, 5],
            Solution::kth_smallest_prime_fraction(vec![1, 2, 3, 5], 3)
        );
    }

    #[test]
    fn nr_786_ex_02() {
        assert_eq!(
            vec![1, 7],
            Solution::kth_smallest_prime_fraction(vec![1, 7], 1)
        );
    }

    #[test]
    fn nr_3075_ex_01() {
        assert_eq!(4, Solution::maximum_happiness_sum(vec![1, 2, 3], 2));
    }

    #[test]
    fn nr_3075_ex_02() {
        assert_eq!(1, Solution::maximum_happiness_sum(vec![1, 1, 1, 1], 2));
    }

    #[test]
    fn nr_3075_ex_03() {
        assert_eq!(5, Solution::maximum_happiness_sum(vec![2, 3, 4, 5], 1));
    }

    #[test]
    fn nr_881_ex_01() {
        assert_eq!(1, Solution::num_rescue_boats(vec![1, 2], 3));
    }

    #[test]
    fn nr_881_ex_02() {
        assert_eq!(3, Solution::num_rescue_boats(vec![3, 2, 2, 1], 3));
    }

    #[test]
    fn nr_881_ex_03() {
        assert_eq!(4, Solution::num_rescue_boats(vec![3, 5, 3, 4], 5));
    }

    #[test]
    fn nr_165_ex_01() {
        assert_eq!(
            0,
            Solution::compare_version("1.01".to_string(), "1.001".to_string())
        );
    }

    #[test]
    fn nr_165_ex_02() {
        assert_eq!(
            0,
            Solution::compare_version("1.0".to_string(), "1.0.0".to_string())
        );
    }

    #[test]
    fn nr_165_ex_03() {
        assert_eq!(
            -1,
            Solution::compare_version("0.1".to_string(), "1.1".to_string())
        );
    }

    #[test]
    fn nr_2997_ex_01() {
        assert_eq!(2, Solution::min_operations_1_lc_l(vec![2, 1, 3, 4], 1));
    }

    #[test]
    fn nr_2997_ex_02() {
        assert_eq!(0, Solution::min_operations_1_lc_l(vec![2, 0, 2, 0], 0));
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
        assert_eq!(
            vec![1i32],
            Solution::find_min_height_trees(4, vec![vec![1, 0], vec![1, 2], vec![1, 3]])
        );
    }

    #[test]
    fn nr_310_ex_02() {
        assert_eq!(
            vec![3, 4i32],
            Solution::find_min_height_trees(
                6,
                vec![vec![3, 0], vec![3, 1], vec![3, 2], vec![3, 4], vec![5, 4]]
            )
        );
    }

    #[test]
    fn nr_752_ex_01() {
        assert_eq!(
            6,
            Solution::open_lock(
                vec![
                    "0201".to_string(),
                    "0101".to_string(),
                    "0102".to_string(),
                    "1212".to_string(),
                    "2002".to_string()
                ],
                "0202".to_string()
            )
        );
    }

    #[test]
    fn nr_752_ex_02() {
        assert_eq!(
            1,
            Solution::open_lock(vec!["8888".to_string()], "0009".to_string())
        );
    }

    #[test]
    fn nr_752_ex_03() {
        assert_eq!(
            -1,
            Solution::open_lock(
                vec![
                    "8887".to_string(),
                    "8889".to_string(),
                    "8878".to_string(),
                    "8898".to_string(),
                    "8788".to_string(),
                    "8988".to_string(),
                    "7888".to_string(),
                    "9888".to_string()
                ],
                "8888".to_string()
            )
        );
    }

    #[test]
    fn nr_1992_ex_01() {
        assert_eq!(
            vec![vec![0, 0, 0, 0], vec![1, 1, 2, 2]],
            Solution::find_farmland(vec![vec![1, 0, 0], vec![0, 1, 1], vec![0, 1, 1]])
        );
    }

    #[test]
    fn nr_1992_ex_02() {
        assert_eq!(
            vec![vec![0, 0, 1, 1]],
            Solution::find_farmland(vec![vec![1, 1], vec![1, 1]])
        );
    }

    #[test]
    fn nr_1992_ex_03() {
        assert_eq!(
            Vec::<Vec<i32>>::new(),
            Solution::find_farmland(vec![vec![0i32]])
        );
    }

    #[test]
    fn nr_200_ex_01() {
        assert_eq!(
            1,
            Solution::num_islands(vec![
                vec!['1', '1', '1', '1', '0'],
                vec!['1', '1', '0', '1', '0'],
                vec!['1', '1', '0', '0', '0'],
                vec!['0', '0', '0', '0', '0']
            ])
        );
    }

    #[test]
    fn nr_200_ex_02() {
        assert_eq!(
            3,
            Solution::num_islands(vec![
                vec!['1', '1', '0', '0', '0'],
                vec!['1', '1', '0', '0', '0'],
                vec!['0', '0', '1', '0', '0'],
                vec!['0', '0', '0', '1', '1']
            ])
        );
    }

    #[test]
    fn nr_129_ex_01() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
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
        assert_eq!(
            vec![2, 13, 3, 11, 5, 17, 7],
            Solution::deck_revealed_increasing(vec![17, 13, 11, 2, 3, 5, 7])
        );
    }

    #[test]
    fn nr_950_ex_02() {
        assert_eq!(
            vec![1, 1000],
            Solution::deck_revealed_increasing(vec![1, 1000])
        );
    }

    #[test]
    fn nr_678_ex_01() {
        assert!(Solution::check_valid_string("()".to_string()));
    }

    #[test]
    fn nr_678_ex_02() {
        assert!(Solution::check_valid_string("(*)".to_string()));
    }

    #[test]
    fn nr_678_ex_03() {
        assert!(Solution::check_valid_string("(*))".to_string()));
    }

    #[test]
    fn nr_1249_ex_01() {
        assert_eq!(
            "lee(t(c)o)de".to_string(),
            Solution::min_remove_to_make_valid("lee(t(c)o)de)".to_string())
        );
    }

    #[test]
    fn nr_1249_ex_02() {
        assert_eq!(
            "ab(c)d".to_string(),
            Solution::min_remove_to_make_valid("a)b(c)d".to_string())
        );
    }

    #[test]
    fn nr_1249_ex_03() {
        assert_eq!(
            "".to_string(),
            Solution::min_remove_to_make_valid("))((".to_string())
        );
    }

    #[test]
    fn nr_79_ex_01() {
        assert!(Solution::exist(
            vec![
                vec!['A', 'B', 'C', 'E'],
                vec!['S', 'F', 'C', 'S'],
                vec!['A', 'D', 'E', 'E']
            ],
            "ABCCED".to_string()
        ));
    }

    #[test]
    fn nr_79_ex_02() {
        assert!(Solution::exist(
            vec![
                vec!['A', 'B', 'C', 'E'],
                vec!['S', 'F', 'C', 'S'],
                vec!['A', 'D', 'E', 'E']
            ],
            "SEE".to_string()
        ));
    }

    #[test]
    fn nr_79_ex_03() {
        assert!(!Solution::exist(
            vec![
                vec!['A', 'B', 'C', 'E'],
                vec!['S', 'F', 'C', 'S'],
                vec!['A', 'D', 'E', 'E']
            ],
            "ABCB".to_string()
        ));
    }

    #[test]
    fn nr_2962_ex_01() {
        assert_eq!(6, Solution::count_subarrays(vec![1, 3, 2, 3, 3], 2));
    }

    #[test]
    fn nr_2962_ex_02() {
        assert_eq!(0, Solution::count_subarrays(vec![1, 4, 2, 1], 3));
    }

    #[test]
    fn nr_2958_ex_01() {
        assert_eq!(
            6,
            Solution::max_subarray_length(vec![1, 2, 3, 1, 2, 3, 1, 2], 2)
        );
    }

    #[test]
    fn nr_2958_ex_02() {
        assert_eq!(
            2,
            Solution::max_subarray_length(vec![1, 2, 1, 2, 1, 2, 1, 2, 1, 2], 1)
        );
    }

    #[test]
    fn nr_2958_ex_03() {
        assert_eq!(4, Solution::max_subarray_length(vec![5, 5, 5, 5, 5, 5], 4));
    }

    #[test]
    fn nr_713_ex_01() {
        assert_eq!(
            8,
            Solution::num_subarray_product_less_than_k(vec![10, 5, 2, 6], 100)
        );
    }

    #[test]
    fn nr_713_ex_02() {
        assert_eq!(
            0,
            Solution::num_subarray_product_less_than_k(vec![1, 2, 3], 0)
        );
    }

    #[test]
    fn nr_442_ex_01() {
        assert_eq!(
            vec![2, 3],
            Solution::find_duplicates(vec![4, 3, 2, 7, 8, 2, 3, 1])
        );
    }

    #[test]
    fn nr_442_ex_02() {
        assert_eq!(vec![1], Solution::find_duplicates(vec![1, 1, 2]));
    }

    #[test]
    fn nr_442_ex_03() {
        assert_eq!(Vec::<i32>::new(), Solution::find_duplicates(vec![1]));
    }

    #[test]
    fn nr_287_ex_01() {
        assert_eq!(2, Solution::find_duplicate(vec![1, 3, 4, 2, 2]));
    }

    #[test]
    fn nr_287_ex_02() {
        assert_eq!(3, Solution::find_duplicate(vec![3, 1, 3, 4, 2]));
    }

    #[test]
    fn nr_287_ex_03() {
        assert_eq!(3, Solution::find_duplicate(vec![3, 3, 3, 3, 3]));
    }

    #[test]
    fn nr_621_ex_01() {
        assert_eq!(
            8,
            Solution::least_interval(vec!['A', 'A', 'A', 'B', 'B', 'B'], 2)
        );
    }

    #[test]
    fn nr_621_ex_02() {
        assert_eq!(
            6,
            Solution::least_interval(vec!['A', 'C', 'A', 'B', 'D', 'B'], 1)
        );
    }

    #[test]
    fn nr_621_ex_03() {
        assert_eq!(
            10,
            Solution::least_interval(vec!['A', 'A', 'A', 'B', 'B', 'B'], 3)
        );
    }

    #[test]
    fn nr_452_ex_01() {
        assert_eq!(
            2,
            Solution::find_min_arrow_shots(vec![vec![10, 16], vec![2, 8], vec![1, 6], vec![7, 12]])
        );
    }

    #[test]
    fn nr_452_ex_02() {
        assert_eq!(
            4,
            Solution::find_min_arrow_shots(vec![vec![1, 2], vec![3, 4], vec![5, 6], vec![7, 8]])
        );
    }

    #[test]
    fn nr_452_ex_03() {
        assert_eq!(
            2,
            Solution::find_min_arrow_shots(vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5]])
        );
    }

    #[test]
    fn nr_57_ex_01() {
        assert_eq!(
            vec![vec![1, 5], vec![6, 9]],
            Solution::insert(vec![vec![1, 3], vec![6, 9]], vec![2, 5])
        );
    }

    #[test]
    fn nr_57_ex_02() {
        assert_eq!(
            vec![vec![1, 2], vec![3, 10], vec![12, 16]],
            Solution::insert(
                vec![
                    vec![1, 2],
                    vec![3, 5],
                    vec![6, 7],
                    vec![8, 10],
                    vec![12, 16]
                ],
                vec![4, 8]
            )
        );
    }

    #[test]
    fn nr_525_ex_01() {
        assert_eq!(2, Solution::find_max_length(vec![0, 1]));
    }

    #[test]
    fn nr_525_ex_02() {
        assert_eq!(2, Solution::find_max_length(vec![0, 1, 0]));
    }

    #[test]
    fn nr_238_ex_01() {
        assert_eq!(
            vec![24, 12, 8, 6],
            Solution::product_except_self(vec![1, 2, 3, 4])
        );
    }

    #[test]
    fn nr_238_ex_02() {
        assert_eq!(
            vec![0, 0, 9, 0, 0],
            Solution::product_except_self(vec![-1, 1, 0, -3, 3])
        );
    }

    #[test]
    fn nr_930_ex_01() {
        assert_eq!(4, Solution::num_subarrays_with_sum(vec![1, 0, 1, 0, 1], 2));
    }

    #[test]
    fn nr_930_ex_02() {
        assert_eq!(15, Solution::num_subarrays_with_sum(vec![0, 0, 0, 0, 0], 0));
    }

    #[test]
    fn nr_791_ex_01() {
        assert_eq!(
            format!("cbad"),
            Solution::custom_sort_string("cba".to_string(), "abcd".to_string())
        );
    }

    #[test]
    fn nr_791_ex_02() {
        assert_eq!(
            format!("bcad"),
            Solution::custom_sort_string("bcafg".to_string(), "abcd".to_string())
        );
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
            left: Some(Rc::new(RefCell::new(TreeNode {
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
        assert_eq!(4, Solution::rob(vec![1, 2, 3, 1]));
    }

    #[test]
    fn nr_198_ex_02() {
        assert_eq!(12, Solution::rob(vec![2, 7, 9, 3, 1]));
    }

    #[test]
    fn nr_931_ex_01() {
        assert_eq!(
            3,
            Solution::min_falling_path_sum(vec![vec![10, 1, 10], vec![1, 10, 10], vec![10, 1, 10]])
        );
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
        assert_eq!(
            vec![vec![1, 2, 10], vec![4, 5, 7, 8]],
            Solution::find_winners(vec![
                vec![1, 3],
                vec![2, 3],
                vec![3, 6],
                vec![5, 6],
                vec![5, 7],
                vec![4, 5],
                vec![4, 8],
                vec![4, 9],
                vec![10, 4],
                vec![10, 9]
            ])
        );
    }

    #[test]
    fn nr_2225_ex_02() {
        assert_eq!(
            vec![vec![1, 2, 5, 6], vec![]],
            Solution::find_winners(vec![vec![2, 3], vec![1, 3], vec![5, 4], vec![6, 4]])
        );
    }

    #[test]
    fn nr_1657_ex_01() {
        assert!(Solution::close_strings(
            "abc".to_string(),
            "bca".to_string()
        ));
    }

    #[test]
    fn nr_1657_ex_02() {
        assert!(!Solution::close_strings("a".to_string(), "aa".to_string()));
    }

    #[test]
    fn nr_1657_ex_03() {
        assert!(Solution::close_strings(
            "cabbba".to_string(),
            "abbccc".to_string()
        ));
    }

    #[test]
    fn nr_1347_ex_01() {
        assert_eq!(1, Solution::min_steps("bab".to_string(), "aba".to_string()));
    }

    #[test]
    fn nr_1347_ex_02() {
        assert_eq!(
            5,
            Solution::min_steps("leetcode".to_string(), "practice".to_string())
        );
    }

    #[test]
    fn nr_1347_ex_03() {
        assert_eq!(
            0,
            Solution::min_steps("anagram".to_string(), "mangaar".to_string())
        );
    }

    #[test]
    fn nr_300_ex_01() {
        assert_eq!(4, Solution::length_of_lis(vec![10, 9, 2, 5, 3, 7, 101, 18]));
    }

    #[test]
    fn nr_300_ex_02() {
        assert_eq!(4, Solution::length_of_lis(vec![0, 1, 0, 3, 2, 3]));
    }

    #[test]
    fn nr_300_ex_03() {
        assert_eq!(1, Solution::length_of_lis(vec![7, 7, 7, 7, 7, 7, 7]));
    }

    #[test]
    fn nr_2870_ex_01() {
        assert_eq!(4, Solution::min_operations(vec![2, 3, 3, 2, 2, 4, 2, 3, 4]));
    }

    #[test]
    fn nr_2870_ex_02() {
        assert_eq!(-1, Solution::min_operations(vec![2, 1, 2, 2, 3, 3]));
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
        assert_eq!(
            4,
            Solution::largest_submatrix(vec![vec![0, 0, 1], vec![1, 1, 1], vec![1, 0, 1]])
        );
    }

    #[test]
    fn nr_1727_largest_submatrix_with_rearrangements_ex_02() {
        assert_eq!(3, Solution::largest_submatrix(vec![vec![1, 0, 1, 0, 1]]));
    }

    #[test]
    fn nr_1727_largest_submatrix_with_rearrangements_ex_03() {
        assert_eq!(
            2,
            Solution::largest_submatrix(vec![vec![1, 1, 0], vec![1, 0, 1]])
        );
    }

    #[test]
    fn nr_1561_maximum_number_of_coins_you_can_get_ex_01() {
        assert_eq!(9, Solution::max_coins(vec![2, 4, 1, 2, 7, 8]));
    }

    #[test]
    fn nr_1561_maximum_number_of_coins_you_can_get_ex_02() {
        assert_eq!(4, Solution::max_coins(vec![2, 4, 5]));
    }

    #[test]
    fn nr_1561_maximum_number_of_coins_you_can_get_ex_03() {
        assert_eq!(18, Solution::max_coins(vec![9, 8, 7, 6, 5, 1, 2, 3, 4]));
    }

    #[test]
    fn nr_1887_reduction_operations_to_make_the_array_elements_equal_ex_01() {
        assert_eq!(3, Solution::reduction_operations(vec![5, 1, 3]));
    }

    #[test]
    fn nr_1887_reduction_operations_to_make_the_array_elements_equal_ex_02() {
        assert_eq!(0, Solution::reduction_operations(vec![1, 1, 1]));
    }

    #[test]
    fn nr_1887_reduction_operations_to_make_the_array_elements_equal_ex_03() {
        assert_eq!(4, Solution::reduction_operations(vec![1, 1, 2, 2, 3]));
    }

    #[test]
    fn nr_1877_minimize_maximum_pair_sum_in_array_ex_01() {
        assert_eq!(7, Solution::min_pair_sum(vec![3, 5, 2, 3]));
    }

    #[test]
    fn nr_1877_minimize_maximum_pair_sum_in_array_ex_02() {
        assert_eq!(8, Solution::min_pair_sum(vec![3, 5, 4, 2, 4, 6]));
    }

    #[test]
    fn nr_1980_find_unique_binary_string_ex_01() {
        assert_eq!(
            "11",
            &Solution::find_different_binary_string(vec![String::from("01"), String::from("10")])
        );
    }

    #[test]
    fn nr_1980_find_unique_binary_string_ex_02() {
        assert_eq!(
            "10",
            &Solution::find_different_binary_string(vec![String::from("00"), String::from("01")])
        );
    }

    #[test]
    fn nr_1980_find_unique_binary_string_ex_03() {
        assert_eq!(
            "000",
            &Solution::find_different_binary_string(vec![
                String::from("111"),
                String::from("011"),
                String::from("001")
            ])
        );
    }

    #[test]
    fn nr_1846_maximum_element_after_decreasing_and_rearranging_ex_01() {
        assert_eq!(
            2,
            Solution::maximum_element_after_decrementing_and_rearranging(vec![2, 2, 1, 2, 1])
        );
    }

    #[test]
    fn nr_1846_maximum_element_after_decreasing_and_rearranging_ex_02() {
        assert_eq!(
            3,
            Solution::maximum_element_after_decrementing_and_rearranging(vec![1, 100, 1000])
        );
    }

    #[test]
    fn nr_1846_maximum_element_after_decreasing_and_rearranging_ex_03() {
        assert_eq!(
            5,
            Solution::maximum_element_after_decrementing_and_rearranging(vec![1, 2, 3, 4, 5])
        );
    }

    #[test]
    fn nr_1930_unique_length3_palindromic_subsequences_ex_01() {
        assert_eq!(
            3,
            Solution::count_palindromic_subsequence(String::from("aabca"))
        );
    }

    #[test]
    fn nr_1930_unique_length3_palindromic_subsequences_ex_02() {
        assert_eq!(
            0,
            Solution::count_palindromic_subsequence(String::from("adc"))
        );
    }

    #[test]
    fn nr_1930_unique_length3_palindromic_subsequences_ex_03() {
        assert_eq!(
            4,
            Solution::count_palindromic_subsequence(String::from("bbcbaba"))
        );
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
        assert_eq!(
            1,
            Solution::eliminate_maximum(vec![1, 1, 2, 3], vec![1, 1, 1, 1])
        );
    }

    #[test]
    fn nr_1921_eliminate_maximum_number_of_monsters_ex_03() {
        assert_eq!(1, Solution::eliminate_maximum(vec![3, 2, 4], vec![5, 3, 2]));
    }

    #[test]
    fn nr_1535_find_the_winner_of_an_array_game_ex_01() {
        assert_eq!(5, Solution::get_winner(vec![2, 1, 3, 5, 4, 6, 7], 2));
    }

    #[test]
    fn nr_1535_find_the_winner_of_an_array_game_ex_02() {
        assert_eq!(3, Solution::get_winner(vec![3, 2, 1], 10));
    }

    #[test]
    fn nr_1503_last_moment_before_all_ants_fall_out_of_a_plank_ex_01() {
        assert_eq!(4, Solution::get_last_moment(4, vec![4, 3], vec![0, 1]));
    }

    #[test]
    fn nr_1503_last_moment_before_all_ants_fall_out_of_a_plank_ex_02() {
        assert_eq!(
            7,
            Solution::get_last_moment(7, vec![], vec![0, 1, 2, 3, 4, 5, 6, 7])
        );
    }

    #[test]
    fn nr_1503_last_moment_before_all_ants_fall_out_of_a_plank_ex_03() {
        assert_eq!(
            7,
            Solution::get_last_moment(7, vec![0, 1, 2, 3, 4, 5, 6, 7], vec![])
        );
    }

    #[test]
    fn nr_1441_build_an_array_with_stack_operations_ex_01() {
        assert_eq!(
            ["Push", "Push", "Pop", "Push"]
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<_>>(),
            Solution::build_array(vec![1, 3], 3)
        );
    }

    #[test]
    fn nr_1441_build_an_array_with_stack_operations_ex_02() {
        assert_eq!(
            ["Push", "Push"]
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<_>>(),
            Solution::build_array(vec![1, 2], 2)
        );
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
        assert_eq!(
            vec![5, 7, 2, 3, 2],
            Solution::find_array(vec![5, 2, 0, 3, 1])
        );
    }

    #[test]
    fn nr_2433_find_the_original_array_of_prefix_xor_ex_02() {
        assert_eq!(vec![13], Solution::find_array(vec![13]));
    }

    #[test]
    fn nr_0005_longest_palindromic_substring_ex_01() {
        assert_eq!(
            String::from("bab"),
            Solution::longest_palindrome(String::from("babad"))
        );
    }

    #[test]
    fn nr_0005_longest_palindromic_substring_ex_02() {
        assert_eq!(
            String::from("bb"),
            Solution::longest_palindrome(String::from("cbbd"))
        );
    }
}
