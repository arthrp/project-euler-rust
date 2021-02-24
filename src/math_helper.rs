pub fn greatest_common_divisor(x1: i128, x2: i128) -> i128 {
    return if x2 == 0 { x1 } else { greatest_common_divisor(x2, x1 % x2) };
}

pub fn lowest_common_multiple(x1: i128, x2: i128) -> i128 {
    return (x1*x2) / greatest_common_divisor(x1, x2);
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