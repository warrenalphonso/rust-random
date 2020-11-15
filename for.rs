fn main() {
    for i in 0..5 {
        println!("{} {}", if i % 2 == 0 {"even"} else {"odd"}, i);
    }
}