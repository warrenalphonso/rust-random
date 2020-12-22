// String type is dynamic and resizable. Programs might also contain string 
// literals which shouldn't be dynamic. Rust allows for both of these. 

fn main() {
    let text = "hey what's up"; // string slice: &str
    let s = text.to_string(); // string literal: String
    println!("{}", text);
    println!("{}", s); 
    // We can coerce s into slice via & 

    // Under the hood, String is Vec<u8> and literals are &[u8]
    let mut s = String::new();
    s.push('H'); // push char 
    s.push_str("ello");
    s.push(' ');
    s += "World! ";
    s.pop(); // can use pop like on Vec 
    println!("{}", s);

    // We can't index strings because they use UTF-8 so characters aren't 
    // necessarily 1 byte. Strings are NOT arrays of Rust's 4-byte chars. 
    // But can do the following
    let beg = &text[0..6];
    println!("{}", beg);

    // .collect() is very general, we need to specify type 
    let sentence = "the quick brown fox jumped over the lazy dog ";
    let words: Vec<&str> = sentence.split_whitespace().collect();
    println!("{:?}", words);
    // For example, don't have to use .collect() to get array 
    let no_spaces: String = sentence.chars().filter(|ch| ! ch.is_whitespace()).collect();
    println!("{}", no_spaces);
}