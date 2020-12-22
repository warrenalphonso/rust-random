// #[derive(Debug)] lets us print structs
#[derive(Debug)]
struct A <'a> {
    // s: &str, // This errors. Must explicitly tell Rust lifetime of reference
    // s: &'static str // This tells Rust the string reference will be valid forever 
    s: &'a str // The reference will be as valid at least as long as the struct exists
}

// Same notation for a string slice returned from a function 
fn forever(i: u32) -> &'static str {
    match i {
        0 => "none", 
        1 => "one", 
        _ => "many"
    }
}

fn main() {
    let a = A { s: "hello" }; 
    println!("{:?}", a);
    let b = forever(3); 
    println!("{}", b);
}