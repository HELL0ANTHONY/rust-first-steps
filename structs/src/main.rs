#[derive(Debug)]
struct User {
    active: bool,
    age: u8,
    email: String,
    name: String,
}

fn new_user(name: String, email: String, age: u8, active: bool) -> User {
    // shorthand como los objetos en js
    User {
        active,
        name,
        age,
        email,
    }
}

fn main() {
    let john = User {
        active: true,
        age: 21,
        email: String::from("john@email.com"),
        name: String::from("John"),
    };
    let emma = new_user(String::from("emma"), String::from("email"), 18, true);

    let joe = User {
        email: String::from("joe@email.com"),
        name: String::from("Joe"),
        // completa el struct con los datos de john
        ..john
    };

    println!("John data: {:?}", john);
    println!("User name: {}", john.name);

    println!("Emma data: {:?}", emma);
    println!("Emma age: {}", emma.age);
    println!("Joe data: {:?}", joe);
    println!("Joe email: {}", joe.email);
    println!("Is active: {}", joe.active);
}
