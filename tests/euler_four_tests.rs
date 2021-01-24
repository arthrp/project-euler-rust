#[path = "../src/euler_4.rs"]
mod euler_4;

#[test]
fn euler_four_solved_correctly() {
    assert_eq!(euler_4::solve(100, 1000), 906609);
}