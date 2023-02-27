#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn check(&self, r: &Rectangle) -> bool {
        self.width > r.width && self.height > r.height
    }
}

// método asociado
impl Rectangle {
    fn cuadrado(l: u32) -> Rectangle {
        Rectangle {
            width: l,
            height: l,
        }
    }
}

fn main() {
    let cuadrado = Rectangle::cuadrado(7);
    println!("cuadrado: {:?}", cuadrado);

    let r = Rectangle {
        width: 35,
        height: 50,
    };

    let r2 = Rectangle {
        width: 45,
        height: 60,
    };

    let r3 = Rectangle {
        width: 30,
        height: 60,
    };

    println!("area: {}", r.area());
    println!("check {}", r2.check(&r));
    println!("check {}", r3.check(&r));
}
