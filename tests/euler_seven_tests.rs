#[path = "../src/euler_7.rs"]
mod euler_7;

#[test]
fn euler_7_solved_correctly(){
    assert_eq!(euler_7::solve(10000), 104743);
}