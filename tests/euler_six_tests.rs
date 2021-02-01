#[path = "../src/euler_6.rs"]
mod euler_6;

#[test]
fn non_func_solved_correctly() {
    assert_eq!(euler_6::solve(), 25164150);
}

#[test]
fn func_solved_correctly() {
    assert_eq!(euler_6::solve_func(), 25164150);
}