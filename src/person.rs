pub struct Person {
    name: String,
    age: u8,
    gender: Gender,
}

#[derive(Clone, Copy)]
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
    pub fn get_age(&self) -> u8 {
        self.age
    }
    pub fn get_gender(&self) -> Gender {
        self.gender
    }
}