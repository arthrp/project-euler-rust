#![allow(unused_parens)]

pub fn solve() -> i32 {
    let mut sum = 0;

    for i in 0..1000 {
        if(i % 3 == 0 || i % 5 == 0){
            sum += i;
        }
    }
    
    sum
}