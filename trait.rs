// Rust structs can't inherit types. They're just data. To create relations 
// between types, we use traits. 

trait Show {
    fn show(&self) -> String;
}

impl Show for i32 {
    fn show(&self) -> String {
        format!("four-byte signed {}", self)
    }
}

impl Show for f64 {
    fn show(&self) -> String {
        format!("eight-byte float {}", self)
    }
}

fn main() {
    println!("{}", 424.show());
    println!("{}", 12.34.show());
}