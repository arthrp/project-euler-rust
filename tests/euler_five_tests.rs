#[path = "../src/euler_five.rs"]
mod euler_five;

#[test]
fn euler_five_solved_correctly() {
    assert_eq!(euler_five::solve(), 232792560);
}