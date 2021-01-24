#![allow(unused_parens)]

pub fn is_palindrome(num: i32) -> bool {
    let str = num.to_string();

    let arr: Vec<char> = str.chars().collect();

    let total_count = arr.len();
    if(total_count == 1){
        return false;
    }

    let max = if(total_count % 2 == 0) { total_count / 2 } else { (total_count-1)/2 };
    
    for i in 0..max {
        if(arr[i] != arr[total_count-i-1]){
            return false;
        }
    }
    
    return true;
}

/*
Taking small advantage of the fact that x*y = y*x so no need to multiply twice
*/
pub fn solve(min: i32, max: i32) -> i32 {
    let mut current_max = 0;

    for x in (min..max) {
        for y in (x..max){
            let product = x * y;
            if(is_palindrome(product) && product > current_max){
                current_max = product;
            }
        }
    }

    return current_max;
}
