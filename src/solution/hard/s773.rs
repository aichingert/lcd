use crate::Solution;

use std::collections::{HashMap, VecDeque};

impl Solution {
    pub fn sliding_puzzle(board: Vec<Vec<i32>>) -> i32 {
        let (mut y, mut x) = (0, 0);

        'out: for (i, r) in board.iter().enumerate() {
            for (j, v) in r.iter().enumerate() {
                if *v == 0 {
                    y = i;
                    x = j;
                    break 'out;
                }
            }
        }

        let mut stack = VecDeque::from([(board, y, x, 0)]);
        let mut map = HashMap::<Vec<Vec<i32>>, i32>::new();

        while let Some((b, y, x, s)) = stack.pop_front() {
            if let Some(&r) = map.get(&b) {
                if r <= s {
                    continue;
                }
            }
            map.insert(b.clone(), s);

            let mut v = true;

            'out: for i in 0..b.len() {
                for j in 0..b[i].len() - i {
                    if b[i][j] != i as i32 * b[0].len() as i32 + j as i32 + 1 {
                        v = false;
                        break 'out;
                    }
                }
            }

            if v {
                return s;
            }

            for (dy, dx) in [(0, 1), (1, 0), (-1, 0), (0, -1)] {
                if y as i32 + dy < 0
                    || x as i32 + dx < 0
                    || y as i32 + dy >= b.len() as i32
                    || x as i32 + dx >= b[0].len() as i32
                {
                    continue;
                }

                let (ny, nx) = (y as i32 + dy, x as i32 + dx);
                let mut cln = b.clone();
                cln[y][x] = cln[ny as usize][nx as usize];
                cln[ny as usize][nx as usize] = 0;
                stack.push_back((cln, ny as usize, nx as usize, s + 1));
            }
        }

        -1
    }
}
