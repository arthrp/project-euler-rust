#![allow(dead_code)]

pub fn solve_func() -> i128 {
    let sum_sqs = get_sum_of_squares_funcional();

    let sq_sum = get_square_of_sum_func();

    return sq_sum - sum_sqs;
}

fn get_sum_of_squares_funcional() -> i128 {
    let nums: Vec<i128> = (1..101).map(|x| i128::pow(x, 2)).collect();

    return nums.into_iter().sum();
}


fn get_square_of_sum_func() -> i128 {
    let sum: i128 = (1..101).sum();

    return i128::pow(sum, 2);
}

//Non-functional style

pub fn solve() -> i128 {
    let sum_sqs = get_sum_of_squares();

    let sq_sum = get_square_of_sum();

    return sq_sum - sum_sqs;
}

fn get_square_of_sum() -> i128 {
    let mut total_sum: i128 = 0;

    for i in 1..101 {
        total_sum += i;
    }

    return i128::pow(total_sum, 2);
}

fn get_sum_of_squares() -> i128 {
    let mut total_sum: i128 = 0;

    for i in 1..101 {
        let n = i128::pow(i, 2);
        total_sum += n;
    }

    return total_sum;
}