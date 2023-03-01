#[derive(Debug)]
pub enum Month {
    January,
    February,
    March,
    April,
    May,
    June,
    July,
    August,
    September,
    October,
    November,
    December,
}

#[derive(Debug)]
pub enum Time {
    Day(Month),
    Hour,
    Minute,
    Second,
}

pub fn second_example() {
    let time = Time::Second;
    let mut counter = 0;

    // with match
    // match time {
    //     Time::Day(month) => println!("{:?}", month),
    //     _ => counter += 1,
    // }
    // println!("counter: {}", counter);

    // if let operator
    if let Time::Day(month) = time {
        println!("{:?}", month)
    } else {
        counter += 1
    }
    println!("counter: {}", counter);
}
