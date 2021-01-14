#[path = "../src/euler_two.rs"]
mod euler_two;

#[test]
fn euler_two_solved_correctly() {
    assert_eq!(euler_two::solve(), 4613732);
}

#[test]
fn euler_two_recursive_solved_correctly() {
    assert_eq!(euler_two::solve_recursive(), 4613732);
}