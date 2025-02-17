pub struct Person {
    name: String,
    age: u8,
    gender: Gender,
}

pub enum Gender {
    Male,
    Female,
    Other,
}

impl Person {
    pub fn new(name: String, age: u8, gender: Gender) -> Person {
        Self { name, age, gender }
    }
    pub fn get_name(&self) -> &String {
        &self.name
    }
}