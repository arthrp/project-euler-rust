#[path = "../src/euler_3.rs"]
mod euler_3;

#[test]
fn is_prime_works() {
    assert_eq!(euler_3::is_prime(113), true);
    assert_eq!(euler_3::is_prime(7), true);
    assert_eq!(euler_3::is_prime(7883), true);
    assert_eq!(euler_3::is_prime(555), false)
}

#[test]
fn euler_three_solved_correctly() {
    assert_eq!(euler_3::solve_pollards_rho(600851475143), 6857);
}

//Ignored since this method is slow
#[test]
#[ignore]
fn euler_three_simple_solved_correctly() {
    assert_eq!(euler_3::solve_simple(600851475143), 6857);
}