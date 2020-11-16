// Vectors are resizable arrays. Unlike slice, we can append to Vec. 
fn main() {
    let mut v = Vec::new();
    v.push(10);
    v.push(20);
    v.push(30);

    let first = v[0]; // Panics if out of range 
    let maybe_first = v.get(0).unwrap_or(&-1); 

    println!("v is {:?}", v);
    println!("first is {}", first);
    println!("maybe_first is {:?}", maybe_first);

    // &v[1..] turns vec into slice
    println!("v is {:?}", v);
    let slice = &v[1..];
    println!("slice is {:?}", slice);
}
