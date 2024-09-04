use crate::Solution;

use std::collections::{HashMap, HashSet};

// NOTE: this code was written at 3 a.m. so be aware of questionable things
impl Solution {
    pub fn robot_sim(commands: Vec<i32>, obstacles: Vec<Vec<i32>>) -> i32 {
        let mut l: HashMap<i32, HashSet<i32>> = HashMap::new();
        let (mut x, mut y) = (0, 0);
        let (mut d, mut a) = (1, 0);

        for obs in obstacles {
            l.entry(obs[0]).and_modify(|s| { s.insert(obs[1]); }).or_insert(HashSet::from_iter([obs[1]]));
        }

        for command in commands {
            match command {
                -1 => d = (d + 1i32).rem_euclid(4),
                -2 => d = (d - 1i32).rem_euclid(4),
                n => {
                    for _ in 0..n {
                        let (px, py) = (x, y);

                        match d {
                            0 => x -= 1,
                            1 => y += 1,
                            2 => x += 1,
                            3 => y -= 1,
                            _ => (),
                        }

                        if l.contains_key(&x) && l[&x].contains(&y) {
                            x = px; y = py;
                        }
                    }

                }
            }

            //println!("{x} {y}");
            a = a.max(x * x + y * y);
        }
        
        a
    }
}
