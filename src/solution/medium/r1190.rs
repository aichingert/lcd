use crate::Solution;

impl Solution {
    pub fn reverse_parentheses(s: String) -> String {
        let mut chs = s.chars().collect::<Vec<_>>();
        let len = chs.len();
        let mut prv = Vec::new();

        for i in 0..len {
            match chs[i] {
                '(' => prv.push(i + 1),
                ')' => {
                    let last = prv.pop().unwrap();
                    let rev = chs[last..i].iter().rev().collect::<String>();

                    for (j, c) in rev.chars().enumerate() {
                        chs[last + j] = c;
                    }
                }
                _ => (),
            }
        }

        chs.into_iter()
            .filter(|&c| !matches!(c, '(' | ')'))
            .collect::<String>()
    }
}
