#[path = "../src/euler_2.rs"]
mod euler_2;

#[test]
fn euler_two_solved_correctly() {
    assert_eq!(euler_2::solve(), 4613732);
}

#[test]
fn euler_two_recursive_solved_correctly() {
    assert_eq!(euler_2::solve_recursive(), 4613732);
}