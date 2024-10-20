use crate::Solution;

impl Solution {
    pub fn parse_bool_expr(expression: String) -> bool {
        let xs = expression.chars().collect::<Vec<_>>();
        Expr::parse(&xs).solve()
    }
}

enum Expr {
    Not(Box<Expr>),
    Or(Vec<Expr>),
    And(Vec<Expr>),
    Bool(bool),
}

impl Expr {
    fn parse(xs: &[char]) -> Self {
        Self::parse_ex(xs, 0).0
    }

    fn parse_ex(xs: &[char], mut p: usize) -> (Expr, usize) {
        match xs[p] {
            '!' => {
                p += 2;
                let (ex, nxt) = Self::parse_ex(xs, p);

                (Expr::Not(Box::new(ex)), nxt + 1)
            }
            '&' | '|' => {
                let op = xs[p];
                p += 2;
                let mut exs = Vec::new();

                while xs[p] != ')' {
                    let (ex, nxt) = Self::parse_ex(xs, p);
                    exs.push(ex);

                    p = nxt;

                    if xs[p] == ',' {
                        p += 1;
                    }
                }

                p += 1;
                match op {
                    '|' => (Expr::Or(exs), p),
                    '&' => (Expr::And(exs), p),
                    _ => unreachable!(),
                }
            }
            't' | 'f' => (Expr::Bool(xs[p] == 't'), p + 1),
            c => panic!("Invalid expression {c:?}"),
        }
    }

    fn solve(&self) -> bool {
        match self {
            Expr::Or(o) => o.iter().fold(false, |acc, c| acc | c.solve()),
            Expr::And(a) => a.iter().fold(true, |acc, c| acc & c.solve()),
            Expr::Not(n) => !n.solve(),
            Expr::Bool(b) => *b,
        }
    }
}
