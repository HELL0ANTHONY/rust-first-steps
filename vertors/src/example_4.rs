pub fn fourth() {
    let mut d = vec![1, 2, 3, 4, 5];

    // change elements in "d"
    for i in &mut d {
        *i += 10;
    }

    for i in &d {
        println!("{}", i)
    }
}
