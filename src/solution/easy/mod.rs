pub mod c3005;
pub mod c70;
pub mod calculate_money_in_leetcode_bank;
pub mod count_of_matches_in_tournament;
pub mod f2485;
pub mod find_mode_in_binary_search_tree;
pub mod i205;
pub mod i349;
pub mod i463;
pub mod l58;
pub mod m1544;
pub mod m1614;
pub mod m2540;
pub mod m2864;
pub mod n1700;
pub mod number_of_1_bits;
pub mod sort_integers_by_the_number_of_1_bits;
pub mod t2073;
pub mod f1971;
pub mod n1137;
pub mod r2000;
pub mod l2441;
pub mod r506;
pub mod l2373;
pub mod e2331;
pub mod s1863;
pub mod s1608;
pub mod s3110;
pub mod r344;
pub mod l409;
pub mod f1002;
pub mod h1051;
pub mod r1122;
pub mod m2037;
pub mod f1791;
pub mod t1550;
pub mod i350;
pub mod p2582;
pub mod w1518;
pub mod c1598;
pub mod l1380;
pub mod s2418;
pub mod s1636;
pub mod n2678;
pub mod m1460;
pub mod k2053;
pub mod k703;
pub mod n476;
pub mod c2022;
pub mod m2220;
pub mod c1684;
pub mod u884;

#[cfg(test)]
mod test {
    use std::cell::RefCell;
    use std::rc::Rc;

    use crate::Solution;
    use crate::TreeNode;

    #[test]
    fn nr_884_ex_01() {
        assert_eq!(
            vec!["sweet".to_string(), "sour".to_string()],
            Solution::uncommon_from_sentences(
                "this apple is sweet".to_string(),
                "this apple is sour".to_string(),
            )
        );
    }

    #[test]
    fn nr_884_ex_02() {
        assert_eq!(
            vec!["banana".to_string()],
            Solution::uncommon_from_sentences(
                "apple apple".to_string(),
                "banana".to_string(),
            )
        );
    }

    #[test]
    fn nr_1684_ex_01() {
        assert_eq!(2, Solution::count_consistent_strings(
                "ab".to_string(), 
                vec!["ad".to_string(), "bd".to_string(), "aaab".to_string(), "baa".to_string(), "badab".to_string()],
        ));
    }

    #[test]
    fn nr_1684_ex_02() {
        assert_eq!(7, Solution::count_consistent_strings(
                "abc".to_string(), 
                vec!["a".to_string(), "b".to_string(), "c".to_string(), "ab".to_string(), "ac".to_string(), "bc".to_string(), "abc".to_string()],
        ));
    }

    #[test]
    fn nr_1684_ex_03() {
        assert_eq!(4, Solution::count_consistent_strings(
                "cad".to_string(), 
                vec!["cc".to_string(), "acd".to_string(), "b".to_string(), "ba".to_string(), "bac".to_string(), "bad".to_string(), "ac".to_string(), "d".to_string()],
        ));
    }

    #[test]
    fn nr_2220_ex_01() {
        assert_eq!(3, Solution::min_bit_flips(10, 7));
    }

    #[test]
    fn nr_2220_ex_02() {
        assert_eq!(3, Solution::min_bit_flips(3, 4));
    }

    #[test]
    fn nr_2022_ex_01() {
        assert_eq!(vec![vec![1,2], vec![3,4]], Solution::construct2_d_array(vec![1,2,3,4], 2, 2));
    }

    #[test]
    fn nr_2022_ex_02() {
        assert_eq!(vec![vec![1,2,3]], Solution::construct2_d_array(vec![1,2,3,], 1, 3));
    }

    #[test]
    fn nr_2022_ex_03() {
        assert_eq!(Vec::<Vec<i32>>::new(), Solution::construct2_d_array(vec![1,2], 1, 1));
    }

    #[test]
    fn nr_476_ex_01() {
        assert_eq!(2, Solution::find_complement(5));
    }

    #[test]
    fn nr_476_ex_02() {
        assert_eq!(1, Solution::find_complement(0));
    }

    #[test]
    fn nr_703_ex_01() {
        let mut kth = crate::solution::easy::k703::KthLargest::new(3, vec![4, 5, 8, 2]);
        let mut actual = Vec::new();
        actual.push(kth.add(3));
        actual.push(kth.add(5));
        actual.push(kth.add(10));
        actual.push(kth.add(9));
        actual.push(kth.add(4));
        assert_eq!(vec![4, 5, 5, 8, 8], actual);
    }

    #[test]
    fn nr_2053_ex_01() {
        assert_eq!("a".to_string(), Solution::kth_distinct(vec!["d".to_string(), "b".to_string(), "c".to_string(), "b".to_string(), "c".to_string(), "a".to_string()], 2));
    }

    #[test]
    fn nr_2053_ex_02() {
        assert_eq!("aaa".to_string(), Solution::kth_distinct(vec!["aaa".to_string(), "aa".to_string(), "a".to_string()], 1));
    }

    #[test]
    fn nr_2053_ex_03() {
        assert_eq!("".to_string(), Solution::kth_distinct(vec!["a".to_string(), "b".to_string(), "a".to_string()], 3));
    }

    #[test]
    fn nr_1460_ex_01() {
        assert_eq!(true, Solution::can_be_equal(vec![1,2,3,4], vec![2,4,1,3])); 
    }

    #[test]
    fn nr_1460_ex_02() {
        assert_eq!(true, Solution::can_be_equal(vec![7], vec![7])); 
    }

    #[test]
    fn nr_1460_ex_03() {
        assert_eq!(false, Solution::can_be_equal(vec![3,7,9], vec![3,7,11])); 
    }

    #[test]
    fn nr_2678_ex_01() {
        assert_eq!(2, Solution::count_seniors(vec!["7868190130M7522".to_string(),"5303914400F9211".to_string(),"9273338290F4010".to_string()])); 
    }

    #[test]
    fn nr_2678_ex_02() {
        assert_eq!(0, Solution::count_seniors(vec!["1313579440F2036".to_string(),"2921522980M5644".to_string()])); 
    }

    #[test]
    fn nr_1636_ex_01() {
        assert_eq!(vec![3,1,1,2,2,2], Solution::frequency_sort_lcd_dup_2(vec![1,1,2,2,2,3]));
    }

    #[test]
    fn nr_1636_ex_02() {
        assert_eq!(vec![1,3,3,2,2], Solution::frequency_sort_lcd_dup_2(vec![2,3,1,3,2]));
    }

    #[test]
    fn nr_1636_ex_03() {
        assert_eq!(vec![5,-1,4,4,-6,-6,1,1,1], Solution::frequency_sort_lcd_dup_2(vec![-1,1,-6,4,5,-6,1,4,1]));
    }

    #[test]
    fn nr_2418_ex_01() {
        assert_eq!(vec!["Mary".to_string(), "Emma".to_string(), "John".to_string()], Solution::sort_people(vec!["Mary".to_string(), "John".to_string(), "Emma".to_string()], vec![180, 165, 170]));
    }

    #[test]
    fn nr_2418_ex_02() {
        assert_eq!(vec!["Alice".to_string(), "Bob".to_string(), "Bob".to_string()], Solution::sort_people(vec!["Bob".to_string(), "Alice".to_string(), "Bob".to_string()], vec![155, 185, 150]));
    }

    #[test]
    fn nr_1380_ex_01() {
        assert_eq!(vec![15], Solution::lucky_numbers(vec![vec![3,7,8],vec![9,11,13],vec![15,16,17]]));
    }
    
    #[test]
    fn nr_1380_ex_02() {
        assert_eq!(vec![12], Solution::lucky_numbers(vec![vec![1,10,4,2],vec![9,3,8,7],vec![15,16,17,12]]));
    }

    #[test]
    fn nr_1380_ex_03() {
        assert_eq!(vec![7], Solution::lucky_numbers(vec![vec![7,8],vec![1,2]]));
    }

    #[test]
    fn nr_1598_ex_01() {
        assert_eq!(2, Solution::min_operations_2_lc_l(vec!["d1/".to_string(), "d2/".to_string(), "../".to_string(), "d21/".to_string(), "./".to_string()]));
    }

    #[test]
    fn nr_1598_ex_02() {
        assert_eq!(3, Solution::min_operations_2_lc_l(vec!["d1/".to_string(), "d2/".to_string(), "./".to_string(), "d3/".to_string(), "../".to_string(), "d31/".to_string()]));
    }


    #[test]
    fn nr_1518_ex_01() {
        assert_eq!(13, Solution::num_water_bottles(9, 3));
    }

    #[test]
    fn nr_1518_ex_02() {
        assert_eq!(19, Solution::num_water_bottles(15, 4));
    }

    #[test]
    fn nr_2582_ex_01() {
        assert_eq!(2, Solution::pass_the_pillow(4, 5));
    }

    #[test]
    fn nr_2582_ex_02() {
        assert_eq!(3, Solution::pass_the_pillow(3, 2));
    }

    #[test]
    fn nr_350_ex_01() {
        assert_eq!(vec![2,2], Solution::intersect(vec![1,2,2,1], vec![2,2,]));
    }
    
    #[test]
    fn nr_350_ex_02() {
        assert_eq!(vec![4,9], Solution::intersect(vec![4,9,5], vec![9,4,9,8,4]));
    }

    #[test]
    fn nr_1550_ex_01() {
        assert!(!Solution::three_consecutive_odds(vec![2,6,4,1]));
    }

    #[test]
    fn nr_1550_ex_02() {
        assert!(Solution::three_consecutive_odds(vec![1,2,34,3,4,5,7,23,12]));
    }

    #[test]
    fn nr_1791_ex_01() {
        assert_eq!(2, Solution::find_center(vec![vec![1,2], vec![2,3], vec![4,2]]));
    }

    #[test]
    fn nr_1791_ex_02() {
        assert_eq!(1, Solution::find_center(vec![vec![1,2], vec![5,1], vec![1,3], vec![1,4]]));
    }

    #[test]
    fn nr_2037_ex_01() {
        assert_eq!(4, Solution::min_moves_to_seat(vec![3,1,5], vec![2,7,4]));
    }

    #[test]
    fn nr_2037_ex_02() {
        assert_eq!(7, Solution::min_moves_to_seat(vec![4,1,5,9], vec![1,3,2,6]));
    }

    #[test]
    fn nr_2037_ex_03() {
        assert_eq!(4, Solution::min_moves_to_seat(vec![2,2,6,6], vec![1,3,2,6]));
    }

    #[test]
    fn nr_1122_ex_01() {
        assert_eq!(vec![2,2,2,1,4,3,3,9,6,7,19], Solution::relative_sort_array(vec![2,3,1,3,2,4,6,7,9,2,19], vec![2,1,4,3,9,6]));
    }

    #[test]
    fn nr_1122_ex_02() {
        assert_eq!(vec![22,28,8,6,17,44], Solution::relative_sort_array(vec![28,6,22,8,44,17], vec![22,28,8,6]));
    }

    #[test]
    fn nr_1051_ex_01() {
        assert_eq!(3, Solution::height_checker(vec![1,1,4,2,1,3]));
    }

    #[test]
    fn nr_1051_ex_02() {
        assert_eq!(5, Solution::height_checker(vec![5,1,2,3,4]));
    }

    #[test]
    fn nr_1051_ex_03() {
        assert_eq!(0, Solution::height_checker(vec![1,2,3,4,5]));
    }

    #[test]
    fn nr_1002_ex_01() {
        assert_eq!(vec!["e".to_string(), "l".to_string(), "l".to_string()], Solution::common_chars(vec!["bella".to_string(), "label".to_string(), "roller".to_string()]));
    }

    #[test]
    fn nr_1002_ex_02() {
        assert_eq!(vec!["c".to_string(), "o".to_string()], Solution::common_chars(vec!["cool".to_string(), "lock".to_string(), "cook".to_string()]));
    }

    #[test]
    fn nr_409_ex_01() {
        assert_eq!(7, Solution::longest_palindrome_dup("abccccdd".to_string()));
    }

    #[test]
    fn nr_409_ex_02() {
        assert_eq!(1, Solution::longest_palindrome_dup("a".to_string()));
    }
   
    #[test]
    fn nr_344_ex_01() {
        let mut rev = vec!['h','e','l','l','o'];
        Solution::reverse_string(&mut rev);
        assert_eq!(vec!['o','l','l','e','h'], rev);
    }

    #[test]
    fn nr_344_ex_02() {
        let mut rev = vec!['H','a','n','n','a', 'h'];
        Solution::reverse_string(&mut rev);
        assert_eq!(vec!['h', 'a','n','n','a','H'], rev);
    }

    #[test]
    fn nr_3110_ex_01() {
        assert_eq!(13, Solution::score_of_string("hello".to_string()));
    }

    #[test]
    fn nr_3110_ex_02() {
        assert_eq!(50, Solution::score_of_string("zaz".to_string()));
    }

    #[test]
    fn nr_1608_ex_01() {
        assert_eq!(2, Solution::special_array(vec![3,5]));
    }

    #[test]
    fn nr_1608_ex_02() {
        assert_eq!(-1, Solution::special_array(vec![0,0]));
    }

    #[test]
    fn nr_1608_ex_03() {
        assert_eq!(3, Solution::special_array(vec![0,4,3,0,4]));
    }

    #[test]
    fn nr_1863_ex_01() {
        assert_eq!(6, Solution::subset_xor_sum(vec![1,3]));
    }

    #[test]
    fn nr_1863_ex_02() {
        assert_eq!(28, Solution::subset_xor_sum(vec![5,1,6]));
    }

    #[test]
    fn nr_1863_ex_03() {
        assert_eq!(480, Solution::subset_xor_sum(vec![3,4,5,6,7,8]));
    }

    #[test]
    fn nr_2331_ex_01() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: Some(Rc::new(RefCell::new(TreeNode::new(0)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
            }))),
        })));

        assert!(Solution::evaluate_tree(root));
    }

    #[test]
    fn nr_2331_ex_02() {
        assert!(!Solution::evaluate_tree(Some(Rc::new(RefCell::new(TreeNode::new(0))))));
    }

    #[test]
    fn nr_2373_ex_01() {
        assert_eq!(vec![vec![9,9], vec![8,6]], Solution::largest_local(vec![vec![9,9,8,1],vec![5,6,2,6],vec![8,2,6,4],vec![6,2,2,2]]));
    }

    #[test]
    fn nr_2373_ex_02() {
        assert_eq!(vec![vec![2,2,2],vec![2,2,2],vec![2,2,2]], Solution::largest_local(vec![vec![1,1,1,1,1],vec![1,1,1,1,1],vec![1,1,2,1,1],vec![1,1,1,1,1],vec![1,1,1,1,1]]));
    }

    #[test]
    fn nr_506_ex_01() {
        let ans = vec!["Gold Medal","Silver Medal","Bronze Medal","4","5"].into_iter().map(|s| s.to_string()).collect::<Vec<_>>();
        assert_eq!(ans, Solution::find_relative_ranks(vec![5,4,3,2,1]));
    }

    #[test]
    fn nr_506_ex_02() {
        let ans = vec!["Gold Medal","5","Bronze Medal","Silver Medal","4"].into_iter().map(|s| s.to_string()).collect::<Vec<_>>();
        assert_eq!(ans, Solution::find_relative_ranks(vec![10,3,8,9,4]));
    }

    #[test]
    fn nr_2441_ex_01() {
        assert_eq!(3, Solution::find_max_k(vec![-1,2,-3,3]));
    }

    #[test]
    fn nr_2441_ex_02() {
        assert_eq!(7, Solution::find_max_k(vec![-1,10,5,7,-7,1]));
    }

    #[test]
    fn nr_2441_ex_03() {
        assert_eq!(-1, Solution::find_max_k(vec![-10,8,6,7,-2,-3]));
    }

    #[test]
    fn nr_2000_ex_01() {
        assert_eq!("dcbaefd".to_string(), Solution::reverse_prefix("abcdefd".to_string(), 'd'));
    }

    #[test]
    fn nr_2000_ex_02() {
        assert_eq!("zxyxxe".to_string(), Solution::reverse_prefix("xyxzxe".to_string(), 'z'));
    }
    
    #[test]
    fn nr_2000_ex_03() {
        assert_eq!("abcd".to_string(), Solution::reverse_prefix("abcd".to_string(), 'z'));
    }

    #[test]
    fn nr_1137_ex_01() {
        assert_eq!(4, Solution::tribonacci(4));
    }

    #[test]
    fn nr_1137_ex_02() {
        assert_eq!(1389537, Solution::tribonacci(25));
    }

    #[test]
    fn nr_1971_ex_01() {
        assert!(Solution::valid_path(3, vec![vec![0,1],vec![1,2],vec![2,0]], 0, 2));
    }

    #[test]
    fn nr_1971_ex_02() {
        assert!(!Solution::valid_path(3, vec![vec![0,1],vec![0,2],vec![3,5],vec![5,4],vec![4,3]], 0, 5));
    }

    #[test]
    fn nr_463_ex_01() {
        assert_eq!(
            16,
            Solution::island_perimeter(vec![
                vec![0, 1, 0, 0],
                vec![1, 1, 1, 0],
                vec![0, 1, 0, 0],
                vec![1, 1, 0, 0]
            ])
        );
    }

    #[test]
    fn nr_463_ex_02() {
        assert_eq!(4, Solution::island_perimeter(vec![vec![1]]));
    }

    #[test]
    fn nr_463_ex_03() {
        assert_eq!(4, Solution::island_perimeter(vec![vec![1, 0]]));
    }

    #[test]
    fn nr_2073_ex_01() {
        assert_eq!(6, Solution::time_required_to_buy(vec![2, 3, 2], 2));
    }

    #[test]
    fn nr_2073_ex_02() {
        assert_eq!(8, Solution::time_required_to_buy(vec![5, 1, 1, 1], 0));
    }

    #[test]
    fn nr_1700_ex_01() {
        assert_eq!(
            0,
            Solution::count_students(vec![1, 1, 0, 0], vec![0, 1, 0, 1])
        );
    }

    #[test]
    fn nr_1700_ex_02() {
        assert_eq!(
            3,
            Solution::count_students(vec![1, 1, 1, 0, 0, 1], vec![1, 0, 0, 0, 1, 1])
        );
    }

    #[test]
    fn nr_1544_ex_01() {
        assert_eq!(
            "leetcode".to_string(),
            Solution::make_good("leEeetcode".to_string())
        );
    }

    #[test]
    fn nr_1544_ex_02() {
        assert_eq!("".to_string(), Solution::make_good("abBAcC".to_string()));
    }

    #[test]
    fn nr_1544_ex_03() {
        assert_eq!("s".to_string(), Solution::make_good("s".to_string()));
    }

    #[test]
    fn nr_1614_ex_01() {
        assert_eq!(3, Solution::max_depth("(1+(2*3)+((8)/4))+1".to_string()));
    }

    #[test]
    fn nr_1614_ex_02() {
        assert_eq!(3, Solution::max_depth("(1)+((2))+(((3)))".to_string()));
    }

    #[test]
    fn nr_205_ex_01() {
        assert!(
            Solution::is_isomorphic("egg".to_string(), "add".to_string())
        );
    }

    #[test]
    fn nr_205_ex_02() {
        assert!(
            !Solution::is_isomorphic("foo".to_string(), "bar".to_string())
        );
    }

    #[test]
    fn nr_205_ex_03() {
        assert!(
            Solution::is_isomorphic("paper".to_string(), "title".to_string())
        );
    }

    #[test]
    fn nr_58_ex_01() {
        assert_eq!(
            5,
            Solution::length_of_last_word(String::from("Hello World"))
        );
    }

    #[test]
    fn nr_58_ex_02() {
        assert_eq!(
            4,
            Solution::length_of_last_word(String::from("   fly me   to   the moon  "))
        );
    }

    #[test]
    fn nr_58_ex_03() {
        assert_eq!(
            6,
            Solution::length_of_last_word(String::from("luffy is still joyboy"))
        );
    }

    #[test]
    fn nr_2485_ex_01() {
        assert_eq!(6, Solution::pivot_integer(8));
    }

    #[test]
    fn nr_2485_ex_02() {
        assert_eq!(1, Solution::pivot_integer(1));
    }

    #[test]
    fn nr_2485_ex_03() {
        assert_eq!(-1, Solution::pivot_integer(4));
    }

    #[test]
    fn nr_349_ex_01() {
        assert_eq!(
            vec![2],
            Solution::intersection(vec![1, 2, 2, 1], vec![2, 2])
        );
    }

    #[test]
    fn nr_349_ex_02() {
        let mut ans = Solution::intersection(vec![4, 9, 5], vec![9, 4, 9, 8, 4]);
        ans.sort_unstable();

        assert_eq!(vec![4, 9], ans);
    }

    #[test]
    fn nr_2540_ex_01() {
        assert_eq!(2, Solution::get_common(vec![1, 2, 3], vec![2, 4]));
    }

    #[test]
    fn nr_2540_ex_02() {
        assert_eq!(2, Solution::get_common(vec![1, 2, 3, 6], vec![2, 3, 4, 5]));
    }

    #[test]
    fn nr_3005_ex_01() {
        assert_eq!(4, Solution::max_frequency_elements(vec![1, 2, 2, 3, 1, 4]));
    }

    #[test]
    fn nr_3005_ex_02() {
        assert_eq!(5, Solution::max_frequency_elements(vec![1, 2, 3, 4, 5]));
    }

    #[test]
    fn nr_2864_ex_01() {
        assert_eq!(
            "001".to_string(),
            Solution::maximum_odd_binary_number("010".to_string())
        );
    }

    #[test]
    fn nr_2864_ex_02() {
        assert_eq!(
            "1001".to_string(),
            Solution::maximum_odd_binary_number("0101".to_string())
        );
    }

    #[test]
    fn nr_70_ex_01() {
        assert_eq!(2, Solution::climb_stairs(2));
    }

    #[test]
    fn nr_70_ex_02() {
        assert_eq!(3, Solution::climb_stairs(3));
    }

    #[test]
    fn nr_1716_calculate_money_in_leetcode_bank_ex_01() {
        assert_eq!(10, Solution::total_money(4));
    }

    #[test]
    fn nr_1716_calculate_money_in_leetcode_bank_ex_02() {
        assert_eq!(37, Solution::total_money(10));
    }

    #[test]
    fn nr_1716_calculate_money_in_leetcode_bank_ex_03() {
        assert_eq!(96, Solution::total_money(20));
    }

    #[test]
    fn nr_1688_count_of_matches_in_tournament_ex_01() {
        assert_eq!(6, Solution::number_of_matches(7));
    }

    #[test]
    fn nr_1688_count_of_matches_in_tournament_ex_02() {
        assert_eq!(13, Solution::number_of_matches(14));
    }

    #[test]
    fn nr_191_number_of_1_bits_ex_01() {
        assert_eq!(3, Solution::hamming_weight(0b01011));
    }

    #[test]
    fn nr_191_number_of_1_bits_ex_02() {
        assert_eq!(1, Solution::hamming_weight(0b0000000010000000));
    }

    #[test]
    fn nr_191_number_of_1_bits_ex_03() {
        assert_eq!(
            31,
            Solution::hamming_weight(0b11111111111111111111111111111101)
        );
    }

    #[test]
    fn nr_0501_find_mode_ex_01() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
                right: None,
            }))),
        })));

        assert_eq!(vec![2], Solution::find_mode(root));
    }

    #[test]
    fn nr_0501_find_mode_ex_02() {
        let root = Some(Rc::new(RefCell::new(TreeNode::new(0))));
        assert_eq!(vec![0], Solution::find_mode(root));
    }

    #[test]
    fn nr_1356_sort_by_bits_ex_01() {
        assert_eq!(
            vec![0, 1, 2, 4, 8, 3, 5, 6, 7],
            Solution::sort_by_bits(vec![0, 1, 2, 3, 4, 5, 6, 7, 8])
        );
    }

    #[test]
    fn nr_1356_sort_by_bits_ex_02() {
        assert_eq!(
            vec![1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024],
            Solution::sort_by_bits(vec![1024, 512, 256, 128, 64, 32, 16, 8, 4, 2, 1])
        );
    }
}
