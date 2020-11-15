use std::f64::consts;

fn main() {
    // let pi: f64 = 3.141562; 
    let pi = consts::PI;
    let x = pi / 2.0; 
    println!("Cosine of {} is {}", x, x.cos());
}