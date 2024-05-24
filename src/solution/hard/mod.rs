pub mod count_vowels_permutation;
pub mod poor_pigs;
pub mod design_graph_with_shortest_path_calculator;
pub mod bus_routes;
pub mod f41;
pub mod f514;
pub mod m1255;

#[cfg(test)]
mod test {
    use crate::Solution;
    use crate::solution::hard::design_graph_with_shortest_path_calculator::Graph;

    #[test]
    fn nr_1255_ex_01() {
        assert_eq!(23, Solution::max_score_words(
            vec!["dog".to_string(),"cat".to_string(),"dad".to_string(),"good".to_string()],
            vec!['a','a','c','d','d','d','g','o','o'],
            vec![1,0,9,5,0,0,3,0,0,0,0,0,0,0,2,0,0,0,0,0,0,0,0,0,0,0],
        ));
    }

    #[test]
    fn nr_1255_ex_02() {
        assert_eq!(27, Solution::max_score_words(
            vec!["xxxz".to_string(),"ax".to_string(),"bx".to_string(),"cx".to_string()],
            vec!['z','a','b','c','x','x','x'],
            vec![4,4,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,5,0,10],
        ));
    }

    #[test]
    fn nr_1255_ex_03() {
        assert_eq!(0, Solution::max_score_words(
            vec!["leetcode".to_string()],
            vec!['l','e','t','c','o','d'],
            vec![0,0,1,1,1,0,0,0,0,0,0,1,0,0,1,0,0,0,0,1,0,0,0,0,0,0],
        ));
    }

    #[test]
    fn nr_514_ex_01() {
        assert_eq!(4, Solution::find_rotate_steps("godding".to_string(), "gd".to_string()));
    }

    #[test]
    fn nr_514_ex_02() {
        assert_eq!(13, Solution::find_rotate_steps("godding".to_string(), "godding".to_string()));
    }

    #[test]
    fn nr_41_ex_01() {
        assert_eq!(3, Solution::first_missing_positive(vec![1,2,0]));
    }

    #[test]
    fn nr_41_ex_02() {
        assert_eq!(2, Solution::first_missing_positive(vec![3,4,-1,1]));
    }

    #[test]
    fn nr_41_ex_03() {
        assert_eq!(1, Solution::first_missing_positive(vec![7,8,9,11,12]));
    }

    #[test]
    fn nr_815_bus_routes_ex_01() {
        assert_eq!(2, Solution::num_buses_to_destination(vec![vec![1,2,7],vec![3,6,7]], 1, 6));
    }

    #[test]
    fn nr_815_bus_routes_ex_02() {
        assert_eq!(-1, Solution::num_buses_to_destination(vec![vec![7,12],vec![4,5,15],vec![6],vec![15,19],vec![9,12,13]], 15, 12));
    }

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
