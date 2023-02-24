fn main() {
    let mut numb: u32 = 5;
    println!("the value of numb is {}", numb);
    numb = 8; // cannot be negative
    println!("the value of numb is {}", numb);

    // boolean
    let mut running = false;
    println!("{}", running);

    running = true;
    println!("{}", running);

    // tuple
    // NOTE: In Rust, tuples have a fixed size and cannot grow or shrink after they have been created.
    let tup = (88, 3.3, "hello");
    let (a, b, c) = tup; // destructuring
    println!("tell me about {:?}", tup);
    println!("a {}. Also can be like it: {}", a, tup.0);
    println!("b {}. Also can be like it: {}", b, tup.1);
    println!("c {}. Also can be like it: {}", c, tup.2);

    // array
    // array of natural numbers
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    println!("array elements {:?}", arr);
    println!("array length {}", arr.len());
    println!("first element of array {}", arr[0]);
    println!("last element of array {}", arr[arr.len() - 1])
}
