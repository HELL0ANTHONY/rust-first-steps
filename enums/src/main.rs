#[derive(Debug)]
enum IP {
    V4,
    V6,
}

#[derive(Debug)]
struct Location {
    location: String,
    ip_type: IP,
}

// de forma más concisa y breve
#[derive(Debug)]
enum OtherIP {
    V4(String),
}

fn main() {
    // let v4 = IP::V4;
    // let v6 = IP::V6;
    let loopback_v4 = Location {
        ip_type: IP::V4,
        location: String::from("127.0.0.1"),
    };
    println!("{:?}", loopback_v4);
    println!("{}", loopback_v4.location);

    let loopback_v6 = Location {
        ip_type: IP::V6,
        location: String::from("127.0.0.1::1"),
    };
    println!("{:?}", loopback_v6);
    println!("{:?}", loopback_v6.ip_type);

    let v4 = OtherIP::V4(String::from("127.0.0.1"));
    println!("{:?}", v4);
}
