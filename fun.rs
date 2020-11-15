fn sqr(x: f64) -> f64 {
    x * x 
}

fn by_ref(x: &i32) -> i32 {
    *x + 1
}

// Function mutates reference - Rust forces you to explicitly acknowledge how 
// dangerous this is 
fn modifies_ref(x: &mut f64) {
    *x = 1.0;
}

fn main() {
    println!("Square is {}", sqr(2.0));

    let i = 10; 
    let res1 = by_ref(&i);
    let res2 = by_ref(&41);
    println!("{} {}", res1, res2);

    let mut res = 0.0; 
    modifies_ref(&mut res);
    println!("{}", res);
}