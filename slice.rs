// Slices are views into an underlying array - we pretend they're just pointers 
// to arrays even though arrays are fixed length and slices can vary. Rust does 
// the work for us. 
// Read &[i32] as "slice of i32" not "address of i32 array"
fn sum(values: &[i32]) -> i32 {
    let mut total = 0; 
    for i in 0..values.len() {
        total += values[i];
    }
    total
}

fn main() {
    let arr = [10, 20, 30, 40]; 
    let sum = sum(&arr); 
    println!("Sum is {}", sum);
}