pub mod sort_integers_by_the_number_of_1_bits;
pub mod find_mode_in_binary_search_tree;
pub mod number_of_1_bits;
pub mod count_of_matches_in_tournament;
pub mod calculate_money_in_leetcode_bank;
pub mod c70;
pub mod m2864;
pub mod c3005;
pub mod m2540;
pub mod i349;
pub mod f2485;
pub mod l58;
pub mod i205;

#[cfg(test)]
mod test {
    use std::rc::Rc;
    use std::cell::RefCell;

    use crate::Solution;
    use crate::TreeNode;

    #[test]
    fn nr_205_ex_01() {
        assert_eq!(true, Solution::is_isomorphic("egg".to_string(), "add".to_string()));
    }

    #[test]
    fn nr_205_ex_02() {
        assert_eq!(false, Solution::is_isomorphic("foo".to_string(), "bar".to_string()));
    }

    #[test]
    fn nr_205_ex_03() {
        assert_eq!(true, Solution::is_isomorphic("paper".to_string(), "title".to_string()));
    }
    
    #[test]
    fn nr_58_ex_01() {
        assert_eq!(5, Solution::length_of_last_word(String::from("Hello World")));
    }

    #[test]
    fn nr_58_ex_02() {
        assert_eq!(4, Solution::length_of_last_word(String::from("   fly me   to   the moon  ")));
    }

    #[test]
    fn nr_58_ex_03() {
        assert_eq!(6, Solution::length_of_last_word(String::from("luffy is still joyboy")));
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
        assert_eq!(vec![2], Solution::intersection(vec![1,2,2,1], vec![2,2]));
    }

    #[test]
    fn nr_349_ex_02() {
        let mut ans = Solution::intersection(vec![4,9,5], vec![9,4,9,8,4]);
        ans.sort_unstable();

        assert_eq!(vec![4,9], ans);
    }

    #[test]
    fn nr_2540_ex_01() {
        assert_eq!(2, Solution::get_common(vec![1,2,3], vec![2,4]));
    }

    #[test]
    fn nr_2540_ex_02() {
        assert_eq!(2, Solution::get_common(vec![1,2,3,6], vec![2,3,4,5]));
    }

    #[test]
    fn nr_3005_ex_01() {
        assert_eq!(4, Solution::max_frequency_elements(vec![1,2,2,3,1,4]));
    }

    #[test]
    fn nr_3005_ex_02() {
        assert_eq!(5, Solution::max_frequency_elements(vec![1,2,3,4,5]));
    }

    #[test]
    fn nr_2864_ex_01() {
        assert_eq!("001".to_string(), Solution::maximum_odd_binary_number("010".to_string()));
    }

    #[test]
    fn nr_2864_ex_02() {
        assert_eq!("1001".to_string(), Solution::maximum_odd_binary_number("0101".to_string()));
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
        assert_eq!(31, Solution::hamming_weight(0b11111111111111111111111111111101));
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
            })))
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
        assert_eq!(vec![0,1,2,4,8,3,5,6,7], Solution::sort_by_bits(vec![0,1,2,3,4,5,6,7,8]));
    }

    #[test]
    fn nr_1356_sort_by_bits_ex_02() {
        assert_eq!(vec![1,2,4,8,16,32,64,128,256,512,1024], Solution::sort_by_bits(vec![1024,512,256,128,64,32,16,8,4,2,1]));
    }
}
