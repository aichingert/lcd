use crate::Solution;

impl Solution {
    pub fn is_circular_sentence(sentence: String) -> bool {
        let chs = sentence.chars().collect::<Vec<_>>();

        if chs[0] != chs[chs.len() - 1] {
            return false;
        }
        let mut p = None::<char>;
        let mut e = false;

        for ch in chs {
            match ch {
                ' ' => e = true,
                c => {
                    if !e {
                        p = Some(c);
                    } else {
                        if p.is_some() && p.unwrap() != c {
                            return false;
                        }

                        p = Some(c);
                        e = false;
                    }
                }
            }
        }

        true
    }
}
