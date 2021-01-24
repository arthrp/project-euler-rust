#[path = "../src/euler_1.rs"]
mod euler_1;

#[test]
fn euler_one_solved_correctly(){
    assert_eq!(euler_1::solve(), 233168);
}