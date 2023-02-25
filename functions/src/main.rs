fn greet() {
    println!("Hello World!")
}

fn my_function(name: &str) {
    println!("hello {}", name);
}

fn add(a: i32, b: i32) -> i32 {
    return a + b; // use the return keyword, but is not needed
}

fn subtraction(a: i32, b: i32) -> i32 {
    // example of ternary operator let a = if x > 5 { 10 } else { 7 };
    if a > b {
        a - b // not use return keyword
    } else {
        b - a
    }
}

fn main() {
    greet();
    my_function("Carmelo");

    let numb = 8;
    let f = {
        let r = numb + 1;
        r + 1
    };
    println!("declaration result {}", f);

    let mut result: i32 = add(5, 5);
    println!("result {}", result);
    result = subtraction(50, 20);
    println!("result {}", result);
}
