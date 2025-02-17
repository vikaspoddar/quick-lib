mod person;

pub fn public_function() {
    println!("called `public_function()`");
}

fn private_function() {
    println!("called `private_function()`");
}

pub fn indirect_access() {
    println!("called  `indirect_access()`, and then call private `private_function()`");
    private_function();
}