pub mod longest_palindromic_substring;
pub mod find_the_original_array_of_prefix_xor;
pub mod count_nodes_equal_to_average_of_subtree;
pub mod build_an_array_with_stack_operations;

#[cfg(test)]
mod test {
    use std::rc::Rc;
    use std::cell::RefCell;

    use crate::TreeNode;
    use crate::Solution;

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
