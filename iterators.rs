fn main() {
    // 0..3 returns an iterator: 
    let mut iter = 0..3;
    assert_eq!(iter.next(), Some(0));
    assert_eq!(iter.next(), Some(1));
    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.next(), None);
    // The above is what for var in iter does 

    // Slices are implicitly converted to iterators
    let arr = [10, 20, 30];
    for i in &arr {
        println!("{}", i);
    }
    // Or we can just call .iter()
    for i in arr.iter() {
        println!("{}", i);
    }

    // Using this to do a sum 
    let sum: i32 = (0..5).sum();
    println!("Sum is {}", sum);
    let sum: i64 = [10, 20, 30, 40].iter().sum();
    println!("sum is {}", sum);
}
