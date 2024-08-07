use crate::Solution;

use std::collections::HashMap;

// TODO: make this better
// I do not like the solution
// neither do I like the problem
impl Solution {
    pub fn number_to_words(mut num: i32) -> String {
        let bases: HashMap<i32, &str> = HashMap::from_iter([
                (1, "One"),
                (2, "Two"),
                (3, "Three"),
                (4, "Four"),
                (5, "Five"),
                (6, "Six"),
                (7, "Seven"),
                (8, "Eight"),
                (9, "Nine"),
                (10, "Ten"),
                (11, "Eleven"),
                (12, "Twelve"),
                (13, "Thirteen"),
                (14, "Fourteen"),
                (15, "Fifteen"),
                (16, "Sixteen"),
                (17, "Seventeen"),
                (18, "Eighteen"),
                (19, "Nineteen"),
                (20, "Twenty"),
                (30, "Thirty"),
                (40, "Forty"),
                (50, "Fifty"),
                (60, "Sixty"),
                (70, "Seventy"),
                (80, "Eighty"),
                (90, "Ninety"),
        ]);


        let hund = get_three(&mut num, &bases);
        let thou = get_three(&mut num, &bases);
        let mill = get_three(&mut num, &bases);
        let bill = get_three(&mut num, &bases);

        let mut ans = String::new();
        ans.push_str(&hund);

        if !thou.is_empty() {
            if ans.is_empty() {
                ans.push_str("Thousand ");
            } else {
                ans.push_str(" Thousand ");
            }
        }
        ans.push_str(&thou);

        if !mill.is_empty() {
            if ans.is_empty() {
                ans.push_str("Million ");
            } else {
                ans.push_str(" Million ");
            }
        }
        ans.push_str(&mill);

        if !bill.is_empty() {
            if ans.is_empty() {
                ans.push_str("Billion ");
            } else {
                ans.push_str(" Billion ");
            }
        }
        ans.push_str(&bill);

        if ans.is_empty() {
            return "Zero".to_string();
        }

        let words = ans.split(' ').rev().collect::<Vec<_>>();
        let mut s = words.into_iter().fold(String::new(), |mut acc, n| {
            acc.push_str(n);
            acc.push(' ');
            acc
        });
        s.pop();
        s
    }
}

fn get_three(n: &mut i32, bases: &HashMap<i32, &str>) -> String {
    let mut ans = String::new();

    let one = *n % 10;
    *n /= 10;

    if one > 0 {
        ans.push_str(bases.get(&one).unwrap());
    }

    let two = *n % 10 * 10;
    *n /= 10;

    if two == 10 {
        ans = bases.get(&(two + one)).unwrap().to_string();
    } else if two > 0 {
        if one != 0 {
            ans.push(' ');
        }
        ans.push_str(bases.get(&two).unwrap());
    }

    let three = *n % 10;
    *n /= 10;

    if three > 0 {
        if ans.is_empty() {
            ans.push_str("Hundred ");
        } else {
            ans.push_str(" Hundred ");
        }
        ans.push_str(bases.get(&three).unwrap());
    }

    ans
}
