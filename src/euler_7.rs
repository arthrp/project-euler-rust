#[path = "../src/math_helper.rs"]
mod math_helper;

pub fn solve(n: u64) -> u64 {
    let arr = sieve_of_eratosthenes(200000);
    let result = arr[n as usize];

    println!("Answer is {}", result);

    return result;
}

fn sieve_of_eratosthenes(max: u64) -> Vec<u64> {
    let mut is_prime: Vec<bool> = Vec::new();
    let mut p: u64 = 2;
    let mut result: Vec<u64> = Vec::new();

    for i in 0..max {
        is_prime.push(true);      
    }

    is_prime[0] = false;
    is_prime[1] = false;

    while(p*p < max){
        if(is_prime[p as usize]){
            let mut i = p*p;
            while(i < max){
                is_prime[i as usize] = false;
                i += p;
            }
        }

        p += 1;
    }

    for i in 1..max {
        if(is_prime[i as usize]){
            println!("Prime {}", i);
            result.push(i);
        }
    }

    return result;
}