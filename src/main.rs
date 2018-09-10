struct Rectangle{
    width: u32,
    height: u32
}

impl Rectangle {
        fn area(&self) -> u32 {
            self.width*self.height
        }
}

fn main() {
    let rect1 = Rectangle {
        width: 10,
        height: 20,
    };

    println!("Area is: {} ", rect1.area());
}
