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

pub fn get_seconds(time: Time) -> u32 {
    match time {
        // también se puede
        /*
            println!("Un segundo no es nada");
            1
        */
        Time::Day(month) => {
            println!("{:?}", month);
            86400
        }
        Time::Hour => 3600,
        Time::Minute => 60,
        Time::Second => 1,
    }
}
