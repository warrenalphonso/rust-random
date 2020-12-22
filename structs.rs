struct Person {
    first_name: String, 
    last_name: String
}

// Simplifying initialization 
impl Person {
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(), 
            last_name: last.to_string(),
        }
    }

    // This is a method since it takes a self argument 
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    // Self is just Person
    fn copy(&self)  -> Self {
        Self::new(&self.first_name, &self.last_name)
    }

    // Allows modifying self 
    fn set_first_name(&mut self, name: &str) {
        self.first_name = name.to_string();
    }

    // Using self *moves* data into the method when called 
    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}

fn main() {
    let p = Person {
        first_name: "John".to_string(), 
        last_name: "Smith".to_string()
    }; 
    println!("{} {}", p.first_name, p.last_name);

    // Initializing a struct that way may be clumsy so we can move the logic 
    // into a function. 
    let p = Person::new("John", "Smith");
    println!("{} {}", p.first_name, p.last_name);

    // Methods 
    println!("{}", p.full_name());

    let mut p = p.copy(); 
    println!("{} {}", p.first_name, p.last_name);

    p.set_first_name("Warren"); 
    println!("{}", p.full_name());

    // Now p is going to be moved so we won't have access to it anymore 
    let tup = p.to_tuple(); 
    println!("{:?}", tup);
    // This errors 
    // println!("{}", p.full_name());
}