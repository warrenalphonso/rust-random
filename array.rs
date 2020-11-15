fn main() {
    // Arrays have their own specific type so it's painful to pass them around as 
    // arguments in Rust. Instead use SLICES. Below has type [i32, 4]
    // I guess this is probably for efficiency
    let arr = [10, 20, 30, 40]; 
    for i in 0..4 {
        println!("{}th element is {}", i, arr[i]);
    }
    println!("Length: {}", arr.len());

    // We can't print an array with {}, have to use a debut print: {:?}
    println!("Array is {:?}", arr);
}