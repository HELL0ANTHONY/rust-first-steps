fn main() {
    let numb: i32 = 7;
    let condition: bool = numb < 10;

    if condition {
        println!("condition has been met")
    } else {
        println!("unfulfilled condition")
    }

    let age: u8 = 18;
    let is_adult: bool = age >= 18;
    // the result of the conditional is assigned to the variable "message"
    let message: &str = if is_adult {
        "is an adult"
    } else {
        "not an adult"
    };
    println!("{}", message);

    // A loop expression denotes an infinite loop.
    let mut count: u8 = 0;
    loop {
        println!("count: {}", count);
        if count == 10 {
            break;
        }
        count += 1;
    }

    let mut flat = 0;
    let result = loop {
        if flat == 10 {
            break flat * flat;
        }
        flat += 1;
    };
    println!("{}", result);

    // while
    let arr = [10, 20, 30, 40, 50, 60, 70, 80];
    let mut i = 0;
    while i < arr.len() {
        println!("position: {}, value: {}", i + 1, arr[i]);
        i += 1;
    }

    for n in arr.iter().rev() {
        println!("value {}", n)
    }

    for n in (0..=8).step_by(2) {
        println!("{}", n)
    }

    // from 0 to 7
    for n in 0..8 {
        println!("{}", n)
    }

    println!("-------");
    // from 0 to 8
    for n in 0..=8 {
        println!("{}", n)
    }
}
