pub mod count_vowels_permutation;
pub mod poor_pigs;
pub mod design_graph_with_shortest_path_calculator;

#[cfg(test)]
mod test {
    use crate::Solution;
    use crate::solution::hard::design_graph_with_shortest_path_calculator::Graph;

    #[test]
    fn nr_2642_design_graph_with_shortest_path_calculator_ex_01() {
        let mut graph = Graph::new(4, vec![vec![0, 2, 5], vec![0, 1, 2], vec![1, 2, 1], vec![3, 0, 3]]);
        assert_eq!(6, graph.shortest_path(3, 2));
        assert_eq!(-1, graph.shortest_path(0, 3));

        graph.add_edge(vec![1,3,4]);

        assert_eq!(6, graph.shortest_path(0, 3));
    }

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
