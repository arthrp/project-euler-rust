pub fn greatest_common_divisor(x1: i128, x2: i128) -> i128 {
    return if x2 == 0 { x1 } else { greatest_common_divisor(x2, x1 % x2) };
}