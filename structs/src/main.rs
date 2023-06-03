#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool { 
        self.width > other.width && self.height > other.height
    }
    fn get_width(&self) -> u32 {
        self.width
    }
    fn get_height(&self) -> u32 {
        self.height
    }
    fn square(size: u32) -> Self {
        Self { width: size, height: size }
    }
}

fn main() {
    let user = User {
        email: String::from("chloevision97@gmail.com"),
        username: String::from("Chloe🌸"),
        sign_in_count: 1,
        active: true, };
    let rect1 = Rectangle { width: 30, height: 50 };  
    let rect2 = Rectangle { width: 10, height: 40 };
    let rect3 = Rectangle { width: 60, height: 45 };
    let sq = Rectangle::square(3);
    println!("The area of the square is {} ", sq.area());
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));  
    println!("{:#?}", user);
    println!("{:#?} and an area of {} ", rect1 , rect1.area());
   println!("The width of the rectangle is {} ", rect1.get_width());
    println!("The height of the rectangle is {} ", rect1.get_height());
}
