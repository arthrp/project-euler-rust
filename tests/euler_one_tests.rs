#[path = "../src/euler_one.rs"]
mod euler_one;

#[test]
fn euler_one_solved_correctly(){
    assert_eq!(euler_one::solve(), 233168);
}