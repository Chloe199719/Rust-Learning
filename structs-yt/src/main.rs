// struct User {
//     username: String,
//     email: String,
//     sign_in_count: u64,
//     active: bool,
// }

// fn main() {
//     let mut user1 = User {
//         email: String::from("chloevision97@gmail.com"),
//         username: String::from("chloevision"),
//         active: true,
//         sign_in_count: 1,
//     };
//     let name = user1.username;
//     user1.username = String::from("Chloe ");
//     let user2 = build_user(String::from("chloevision98@gmail.com"), String::from("Chloe"));
//     let user3 = User {
//         email: String::from("email@email.com"),
//         username: String::from("username"),
//         ..user2 // struct update syntax,
//     };
// }

// fn build_user(email: String, username: String) -> User {
//     User {
//         email, // field init shorthand
//         username,
//         active: true,
//         sign_in_count: 1,
//     }
// }

// fn tuple_structs() {
//     struct Color(i32, i32, i32);
//     struct Point(i32, i32, i32);
//     let black = Color(0, 0, 0);
//     let origin = Point(0, 0, 0);
// }
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn  area(&self ) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
    fn create(width: u32, height: u32) -> Rectangle {
        Rectangle {
            width,
            height,
        }
    }
}
fn main(){
    let rect = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 40,
        height: 50,
    };
    let rect1 = Rectangle {
        width: 20,
        height: 40,
    };
    let rect3 = Rectangle::square(3);
    let rect4 = Rectangle::create(3, 4);
    println!("rect3 is {:#?}", rect3);
    println!("rect4 is {:#?}", rect4);
    println!("Can rect hold rect2? {}", rect.can_hold(&rect2));
    println!("Can rect hold rect1? {}", rect.can_hold(&rect1));
    println!("rect is {:#?}", rect);
    println!("The area of the rectangle is {} square pixels.", rect.area());
}

