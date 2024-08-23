use crate::Solution;

// TODO: this could be done better
impl Solution {
    pub fn fraction_addition(expression: String) -> String {
        let expr = expression.bytes().collect::<Vec<_>>();
        let mut i = 0;
        let mut is_neg = expr[i] == b'-';
        if is_neg { i += 1; }

        let mut num = get_next(&mut i, &expr);
        i += 1; // /n
        let mut den = get_next(&mut i, &expr);

        while i < expr.len() {
            let op = expr[i];
            i += 1;

            let mut onum = get_next(&mut i, &expr);
            i += 1;
            let oden = get_next(&mut i, &expr);

            match op {
                b'+' => {
                    let mut next = den;
                    if den != oden {
                        next = den * oden;
                        num *= oden;
                        onum *= den; 
                    }

                    if is_neg {
                        num -= onum;
                    } else {
                        num += onum;
                    }

                    if is_neg && num < 0 {
                        is_neg = false;
                    }
                    num = num.abs();
                    den = next;
                }
                b'-' => {
                    let mut next = den;
                    if den != oden {
                        next = den * oden;
                        num *= oden;
                        onum *= den;
                    }

                    if is_neg {
                        num += onum;
                    } else {
                        num -= onum;
                    }

                    if !is_neg && num < 0 {
                        is_neg = true;
                    }
                    num = num.abs();
                    den = next;
                }
                _ => panic!("invalid op {}", op as char),
            }
        }
        
        let mut short = 2;
        while short < num + den {
            if num % short == 0 && den % short == 0 {
                den /= short;
                num /= short;
            } else {
                short += 1;
            }
        }

        while num != 0 && num != 1 && den % num == 0 {
            den /= num;
            num /= num;
        }
        while den != 0 && den != 1 && num % den == 0 {
            num /= den;
            den /= den;
        }

        if num == 0 { is_neg = false; den = 1; }

        format!("{}{num}/{den}", if is_neg { "-" } else { "" })
    }
}

fn get_next(i: &mut usize, expr: &[u8]) -> i32 {
    let mut n = 0;
    while *i < expr.len() && expr[*i] >= b'0' && expr[*i] <= b'9' {
        n *= 10;
        n += (expr[*i] - b'0') as i32;
        *i += 1;
    }

    n
}
