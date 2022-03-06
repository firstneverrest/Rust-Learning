// implement field (no function inside struct)
pub struct Person {
    name: String,
    age: u8,
}

// implement behavior
impl Person {
    pub fn new(name: String, age: u8) -> Self {
        Self {
            name: name,
            age: age,
        }
    }

    // must use &self (borrow) to make it be function 
    // that can access from object not module
    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_age(&self) -> u8 {
        self.age
    }
}