pub mod longest_palindromic_substring;
pub mod find_the_original_array_of_prefix_xor;
pub mod count_nodes_equal_to_average_of_subtree;
pub mod build_an_array_with_stack_operations;
pub mod last_moment_before_all_ants_fall_out_of_a_plank;
pub mod find_the_winner_of_an_array_game;
pub mod eliminate_maximum_number_of_monsters;
pub mod determine_if_a_cell_is_reachable_at_a_given_time;

#[cfg(test)]
mod test {
    use std::rc::Rc;
    use std::cell::RefCell;

    use crate::TreeNode;
    use crate::Solution;

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
