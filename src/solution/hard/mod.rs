pub mod bus_routes;
pub mod count_vowels_permutation;
pub mod design_graph_with_shortest_path_calculator;
pub mod f41;
pub mod f514;
pub mod f719;
pub mod i273;
pub mod i502;
pub mod m1255;
pub mod m1568;
pub mod m995;
pub mod n726;
pub mod p330;
pub mod poor_pigs;
pub mod r2751;
pub mod s552;
pub mod w140;

#[cfg(test)]
mod test {
    use crate::solution::hard::design_graph_with_shortest_path_calculator::Graph;
    use crate::Solution;

    #[test]
    fn nr_719_ex_01() {
        assert_eq!(0, Solution::smallest_distance_pair(vec![1, 3, 1], 1));
    }

    #[test]
    fn nr_719_ex_02() {
        assert_eq!(0, Solution::smallest_distance_pair(vec![1, 1, 1], 2));
    }

    #[test]
    fn nr_719_ex_03() {
        assert_eq!(5, Solution::smallest_distance_pair(vec![1, 6, 1], 3));
    }

    #[test]
    fn nr_1568_ex_01() {
        assert_eq!(
            2,
            Solution::min_days_lc_1(vec![vec![0, 1, 1, 0], vec![0, 1, 1, 0], vec![0, 0, 0, 0]])
        );
    }

    #[test]
    fn nr_1568_ex_02() {
        assert_eq!(2, Solution::min_days_lc_1(vec![vec![1, 1]]));
    }

    #[test]
    fn nr_273_ex_01() {
        assert_eq!(
            "One Hundred Twenty Three".to_string(),
            Solution::number_to_words(123)
        );
    }

    #[test]
    fn nr_273_ex_02() {
        assert_eq!(
            "Twelve Thousand Three Hundred Forty Five".to_string(),
            Solution::number_to_words(12345)
        );
    }

    #[test]
    fn nr_273_ex_03() {
        assert_eq!(
            "One Million Two Hundred Thirty Four Thousand Five Hundred Sixty Seven".to_string(),
            Solution::number_to_words(1234567)
        );
    }

    #[test]
    fn nr_726_ex_01() {
        assert_eq!(
            "H2O".to_string(),
            Solution::count_of_atoms("H2O".to_string())
        );
    }

    #[test]
    fn nr_726_ex_02() {
        assert_eq!(
            "H2MgO2".to_string(),
            Solution::count_of_atoms("Mg(OH)2".to_string())
        );
    }

    #[test]
    fn nr_726_ex_03() {
        assert_eq!(
            "K4N2O14S4".to_string(),
            Solution::count_of_atoms("K4(ON(SO3)2)2".to_string())
        );
    }

    #[test]
    fn nr_2751_ex_01() {
        assert_eq!(
            vec![2, 17, 9, 15, 10],
            Solution::survived_robots_healths(
                vec![5, 4, 3, 2, 1],
                vec![2, 17, 9, 15, 10],
                String::from("RRRRR")
            )
        );
    }

    #[test]
    fn nr_2751_ex_02() {
        assert_eq!(
            vec![14],
            Solution::survived_robots_healths(
                vec![3, 5, 2, 6],
                vec![10, 10, 15, 12],
                String::from("RLRL")
            )
        );
    }

    #[test]
    fn nr_2751_ex_03() {
        assert_eq!(
            Vec::<i32>::new(),
            Solution::survived_robots_healths(
                vec![1, 2, 5, 6],
                vec![10, 10, 11, 11],
                String::from("RLRL")
            )
        );
    }

    #[test]
    fn nr_995_ex_01() {
        assert_eq!(2, Solution::min_k_bit_flips(vec![0, 1, 0], 1));
    }

    #[test]
    fn nr_995_ex_02() {
        assert_eq!(-1, Solution::min_k_bit_flips(vec![1, 1, 0], 2));
    }

    #[test]
    fn nr_995_ex_03() {
        assert_eq!(
            3,
            Solution::min_k_bit_flips(vec![0, 0, 0, 1, 0, 1, 1, 0], 3)
        );
    }

    #[test]
    fn nr_330_ex_01() {
        assert_eq!(1, Solution::min_patches(vec![1, 3], 6));
    }

    #[test]
    fn nr_330_ex_02() {
        assert_eq!(2, Solution::min_patches(vec![1, 5, 10], 20));
    }

    #[test]
    fn nr_330_ex_03() {
        assert_eq!(0, Solution::min_patches(vec![1, 2, 2], 5));
    }

    #[test]
    fn nr_502_ex_01() {
        assert_eq!(
            4,
            Solution::find_maximized_capital(2, 0, vec![1, 2, 3], vec![0, 1, 1])
        );
    }

    #[test]
    fn nr_502_ex_02() {
        assert_eq!(
            6,
            Solution::find_maximized_capital(3, 0, vec![1, 2, 3], vec![0, 1, 2])
        );
    }

    #[test]
    fn nr_552_ex_01() {
        assert_eq!(8, Solution::check_record(2));
    }

    #[test]
    fn nr_552_ex_02() {
        assert_eq!(3, Solution::check_record(1));
    }

    #[test]
    fn nr_552_ex_03() {
        assert_eq!(183236316, Solution::check_record(10101));
    }

    #[test]
    fn nr_140_ex_01() {
        let expect = ["cats and dog".to_string(), "cat sand dog".to_string()];

        for result in Solution::word_break(
            "catsanddog".to_string(),
            vec![
                "cat".to_string(),
                "cats".to_string(),
                "and".to_string(),
                "sand".to_string(),
                "dog".to_string(),
            ],
        ) {
            assert!(expect.contains(&result));
        }
    }

    #[test]
    fn nr_140_ex_02() {
        let expect = [
            "pine apple pen apple".to_string(),
            "pineapple pen apple".to_string(),
            "pine applepen apple".to_string(),
        ];

        for result in Solution::word_break(
            "pineapplepenapple".to_string(),
            vec![
                "apple".to_string(),
                "pen".to_string(),
                "applepen".to_string(),
                "pine".to_string(),
                "pineapple".to_string(),
            ],
        ) {
            assert!(expect.contains(&result));
        }
    }

    #[test]
    fn nr_140_ex_03() {
        assert_eq!(
            Vec::<String>::new(),
            Solution::word_break(
                "catsandog".to_string(),
                vec![
                    "cats".to_string(),
                    "dog".to_string(),
                    "sand".to_string(),
                    "and".to_string(),
                    "cat".to_string()
                ]
            )
        );
    }

    #[test]
    fn nr_1255_ex_01() {
        assert_eq!(
            23,
            Solution::max_score_words(
                vec![
                    "dog".to_string(),
                    "cat".to_string(),
                    "dad".to_string(),
                    "good".to_string()
                ],
                vec!['a', 'a', 'c', 'd', 'd', 'd', 'g', 'o', 'o'],
                vec![1, 0, 9, 5, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            )
        );
    }

    #[test]
    fn nr_1255_ex_02() {
        assert_eq!(
            27,
            Solution::max_score_words(
                vec![
                    "xxxz".to_string(),
                    "ax".to_string(),
                    "bx".to_string(),
                    "cx".to_string()
                ],
                vec!['z', 'a', 'b', 'c', 'x', 'x', 'x'],
                vec![4, 4, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 10],
            )
        );
    }

    #[test]
    fn nr_1255_ex_03() {
        assert_eq!(
            0,
            Solution::max_score_words(
                vec!["leetcode".to_string()],
                vec!['l', 'e', 't', 'c', 'o', 'd'],
                vec![0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0],
            )
        );
    }

    #[test]
    fn nr_514_ex_01() {
        assert_eq!(
            4,
            Solution::find_rotate_steps("godding".to_string(), "gd".to_string())
        );
    }

    #[test]
    fn nr_514_ex_02() {
        assert_eq!(
            13,
            Solution::find_rotate_steps("godding".to_string(), "godding".to_string())
        );
    }

    #[test]
    fn nr_41_ex_01() {
        assert_eq!(3, Solution::first_missing_positive(vec![1, 2, 0]));
    }

    #[test]
    fn nr_41_ex_02() {
        assert_eq!(2, Solution::first_missing_positive(vec![3, 4, -1, 1]));
    }

    #[test]
    fn nr_41_ex_03() {
        assert_eq!(1, Solution::first_missing_positive(vec![7, 8, 9, 11, 12]));
    }

    #[test]
    fn nr_815_bus_routes_ex_01() {
        assert_eq!(
            2,
            Solution::num_buses_to_destination(vec![vec![1, 2, 7], vec![3, 6, 7]], 1, 6)
        );
    }

    #[test]
    fn nr_815_bus_routes_ex_02() {
        assert_eq!(
            -1,
            Solution::num_buses_to_destination(
                vec![
                    vec![7, 12],
                    vec![4, 5, 15],
                    vec![6],
                    vec![15, 19],
                    vec![9, 12, 13]
                ],
                15,
                12
            )
        );
    }

    #[test]
    fn nr_2642_design_graph_with_shortest_path_calculator_ex_01() {
        let mut graph = Graph::new(
            4,
            vec![vec![0, 2, 5], vec![0, 1, 2], vec![1, 2, 1], vec![3, 0, 3]],
        );
        assert_eq!(6, graph.shortest_path(3, 2));
        assert_eq!(-1, graph.shortest_path(0, 3));

        graph.add_edge(vec![1, 3, 4]);

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
