fn dump(s: String) {
    // Now dump has ownership of s 
    println!("{}", s);
}

fn main() {
    let s1 = "Call me Warren".to_string();
    let s2 = s1; 
    // This errors: s1 no longer has *ownership* of the string, s2 does 
    // println!("s1 {}", s1);
    println!("s2 {}", s2);
    // Also happens for functions since we pass ownership 
    dump(s2);
    // Errors
    // println!("s2?: {}", s2);
}