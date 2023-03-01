pub fn second() {
    // other way to create a vector
    let new_vector = vec![1, 2, 3, 4, 5];
    let element: &i32 = &new_vector[2];
    println!("new vector: {:?}", new_vector);
    println!("third element: {}", element);

    match new_vector.get(2) {
        Some(third) => println!("this is the third element: {}", third),
        None => println!("is not the third element"),
    }
}
