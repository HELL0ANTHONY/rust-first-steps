pub fn first_example() {
    let max_configuration = Some(7u8);
    // with match
    match max_configuration {
        Some(max) => println!("the max is {}", max),
        _ => (),
    }

    // with "if let operator"
    // más corto
    if let Some(max) = max_configuration {
        println!("the max is {}", max)
    }
}

