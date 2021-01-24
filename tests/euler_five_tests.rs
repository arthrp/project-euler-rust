#[path = "../src/euler_5.rs"]
mod euler_5;

#[test]
fn euler_five_solved_correctly() {
    assert_eq!(euler_5::solve(), 232792560);
}