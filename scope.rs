// Rust errors if you have a stale pointer! 
fn main() {
    let s1 = "first string".to_string(); 
    let mut rs1 = &s1;
    { 
        let tmp = "second string".to_string();
        // tmp ONLY exists in this block - its resources are FREED after  
        rs1 = &tmp;
    }
    // Errors because tmp was freed! 
    println!("ref {}", rs1);
}