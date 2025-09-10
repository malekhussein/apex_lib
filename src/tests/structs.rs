pub struct Person {
    pub name: String,
    pub age: u8,
}


impl Person {
    pub fn new(name: &str, age: u8) -> Self {
        Person {
            name: name.to_string(),
            age,
        }
    }

    pub fn greet(&self) {
        println!("Hello, my name is {} and I am {} years old", self.name, self.age);
    }
}
