#![allow(unused_parens)]
#![allow(dead_code)]

pub fn solve() -> i64 {
    return get_fibonacci_iterative(4000000);
}

pub fn solve_recursive() -> i64 {
    return get_fibonacci_sum_recursive(0, 1, 4000000);
}

fn get_fibonacci_iterative(max: i64) -> i64 {
    let mut prev = 0;
    let mut curr = 1;
    let mut sum = 0;

    while(curr < max){
        if (curr % 2 == 0){
            sum += curr;
        }

        let tmp_prev = prev;

        prev = curr;
        curr = tmp_prev + curr;
    }

    return sum;
}

fn get_fibonacci_sum_recursive(prev: i64, curr: i64, max: i64) -> i64 {
    if(curr >= max){
        return 0;
    }

    if(curr % 2 == 0){
        return curr + get_fibonacci_sum_recursive(curr, curr+prev, max);
    }
    else{
        return 0 + get_fibonacci_sum_recursive(curr, curr+prev, max);
    }
}