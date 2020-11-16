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

    // Use vec! as shorthand for initialization 
    let mut v1 = vec![5,4,3];
    let mut v2 = Vec::new();
    v2.push(5);
    v2.push(4);
    v2.push(3);
    // Comparisons are by value
    assert_eq!(v1, v2);

    v1.extend(0..2);
    assert_eq!(v1, &[5,4,3,0, 1]);
    println!("Final v1 is {:?}", v1);

    // Inserting into a vector is inefficient since Rust then has to shift values 
    v1.insert(3, 2);
    v1.insert(4, 1);
    v1.remove(6);
    println!("New v1 is {:?}", v1);

    // We can sort then remove duplicates in-place: 
    let mut v3 = vec![1, 2, 3, 4, 1, 2, 3, 8, 2, 3, 2, 5]; 
    v3.sort();
    v3.dedup();
    println!("v3: {:?}", v3);
}
