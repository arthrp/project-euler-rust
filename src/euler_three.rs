#![allow(unused_parens)]

use std::num::Wrapping;

/* 
One way to solve the largest factor problem is to use Pollard's Rho algorithm which returns all the factors of a number. 
Then we choose the largest one and return it as answer.
Obvious drawback is that we need only the largest factor and will discard all the others calculated.
But it's actually much faster than 'simple' method below
*/
pub fn solve_pollards_rho(num: i128) -> i128 {
    let mut acc = Vec::new();
    get_factors(&mut acc, num);

    let largest = acc.iter().fold(acc[0], |max, x| { if max > *x { max } else { *x } });
    return largest
}

/* 
Other way is to remember that if solution exists then we will have an equation like num = x * k
We're starting from k = 1 and as soon as we get k where n/k is integer and prime number, we found our solution. 
*/
pub fn solve_simple(num: i64) -> i64 {
    for k in 1..num/2 {
        if(num % k == 0 && is_prime((num/k) as i128)){
            return num/k;
        }
    }

    return num
}

pub fn is_prime(num: i128) -> bool {
    let max = (num as f64).sqrt() as i128;
    for i in 2..max {
        if(num % i == 0){
            return false;
        }
    }

    return true;
}

fn get_factors(acc: &mut Vec<i128>, num: i128) -> () {
    if(num == 1){
        return;
    }

    if(is_prime(num)){
        acc.push(num);
        return;
    }

    let divisor = rho(num);
    get_factors(acc, divisor);
    get_factors(acc, num/divisor);
}

fn rho (num: i128) -> i128 {
    let mut x1 = 2;
    let mut x2 = 2;
    let mut divisor;

    if (num % 2 == 0){
        return 2;
    }

    loop {
        x1 = func(x1) % num;
        x2 = func(func(x2)) % num;
        divisor = greatest_common_divisor((x1 - x2).abs(), num);
        if(divisor != 1){
            break;
        }
    }

    return divisor;
}

fn greatest_common_divisor(x1: i128, x2: i128) -> i128 {
    return if x2 == 0 { x1 } else { greatest_common_divisor(x2, x1 % x2) };
}

fn func(n: i128) -> i128 {
    // i128 will overflow and it's ok
    let wn = Wrapping(n);
    return (wn * wn + Wrapping(1)).0;
}