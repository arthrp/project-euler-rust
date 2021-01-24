#![allow(unused_parens)]

//Wtf, rust. Why it works in euler_three and not here?
#[path = "../src/math_helper.rs"]
mod math_helper;

pub fn solve() -> i128 {
    let mut acc = 1;
    for i in 1..20 {
        acc = math_helper::lowest_common_multiple(acc, i)
    }
    return acc;
}

