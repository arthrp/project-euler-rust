#[path = "../src/euler_four.rs"]
mod euler_four;

#[test]
fn euler_four_solved_correctly() {
    assert_eq!(euler_four::solve(100, 1000), 906609);
}