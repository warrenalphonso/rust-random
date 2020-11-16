use std::env::args;

fn main() {
    for arg in args() {
        println!("'{}'", arg);
    }

    let v: Vec<String> = args().skip(1).collect();
    for arg in v {
        println!("{}", arg);
    }

    // Ask for arguments if none provided 
    let first = args().nth(1).expect("Re-run with arguments please.");
    // If arg, convert to type i32. If cannot, yell at user 
    let n: i32 = first.parse().expect("Not an integer...");
    println!("{}", n);
}