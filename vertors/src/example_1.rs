pub fn first() {
    let mut v: Vec<i32> = Vec::new(); // empty
    v.push(2);
    v.push(3);
    v.push(5 - 1);
    v.push(2 + 3);
    println!("length: {}", v.len());
    println!("vector: {:?}", v);

    v.pop();
    println!("length: {}", v.len());
    println!("vector: {:?}", v);
}
