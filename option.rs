fn main() {
    let ints: [i32; 5] = [1,2,3,4,5];
    let slice = &ints;
    // Using .get() on slices returns Some if we use a valid index, None if not
    let first = slice.get(0);
    let maybe_last = slice.get(5);
    // We can use .unwrap() to get what's inside Some 
    println!("First: {:?}", first.unwrap());

    // Important: Some contains a pointer: 
    let last = if maybe_last.is_some() {
        *maybe_last.unwrap() // We dereference
    } else {
        -1
    };

    println!("Maybe last: {:?}", last);

    // Shortcut to doing above: unwrap_or() uses argument as return if None 
    let easy_last = *slice.get(5).unwrap_or(&-1);
    println!("Maybe last: {:?}", easy_last);
}