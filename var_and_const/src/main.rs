fn print_type_of<T>(_: &T) {
    println!("the type of the variable is {}", std::any::type_name::<T>())
}

fn main() {
    // no mutable
    let number: i32 = 3;
    println!("the number is: {}", number);
    // use "mut" to make a variable mutable
    let mut mutable_number: i32 = 4;
    println!("the mutable number first is {}", mutable_number);
    mutable_number = mutable_number * 3;
    println!("now the mutable is equal to {}", mutable_number);

    // constant
    /*
       - in a const we cannot use the keyword "mut"
       - need to declare the type
       - use upper case to the declare a const
    */
    const PI: f32 = 3.1416;
    println!("PI: {}", PI);

    // using shadowing
    let string_numb = 7; // rust infers the type
    print_type_of(&string_numb);

    // se crea una nueva variable con el mismo nombre, pero con un tipo diferente
    let string_numb = "hello";
    print_type_of(&string_numb);
}
