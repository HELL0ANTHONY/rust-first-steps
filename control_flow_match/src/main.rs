// mod first_example;
// use first_example::Month;
// use first_example::Time;

mod second_example;

fn main() {
    // first_example
    // let day_of_january = Time::Day(Month::January);
    // let time_on_seconds = first_example::get_seconds(day_of_january);
    // println!("{}", time_on_seconds);

    // second_example
    let five = Some(5);
    let six = second_example::add_one(five);
    let none = second_example::add_one(None);
    println!("five: {:?}, six: {:?}, none: {:?}", five, six, none);
}
