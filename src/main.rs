#![allow(unused_parens)]

mod euler_four;

fn main() {
    // let y = euler_three::solve_pollards_rho(600851475143);

    // println!("Result is {}", y);
    let x = euler_four::solve(100,1000);
    println!("Result is {}", x);
}
