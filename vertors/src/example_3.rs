// error
pub fn third() {
    let other = vec![1, 2, 3, 4, 5];
    // this return a error
    // let non_existen_element = &other[500];
    // println!("return an error: {}", non_existen_element); // thread 'main' panicked at 'index out of bounds: the len is 5 but the index is 500', src/example_3.rs:4:32

    // this return a None
    let non_existen_element = other.get(500);
    println!("return None, {:?}", non_existen_element); // None
}
