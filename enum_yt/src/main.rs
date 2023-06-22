fn main() {
//    enum  Option<T> {
//        Some(T),
//        None,
//    }
//    let some_number = Some(5);
//    let some_string = Some("a string");
//    let absent_number: Option<i32> = None;       
//    let x: i8 = 5;
//    let y: Option<i8> = Some(5);
//    let sum = x + y.unwrap_or(0);
    //     println!("sum is {}", sum)
//  let coin1 = Coin::Quarter(UsState::Alaska);
//  println!("coin1 is {}", value_in_cents(coin1));
    // let five = Some(5);
    // let six = plus_one(five);
    // let none = plus_one(None);
    // print!("six is {:?} and none is {:?} \n", six, none);

    let some_value = Some(3);
    match some_value {
        Some(3) => println!("three"),
        _ => (),
    }
        
    }


// fn plus_one(x: Option<i32>) -> Option<i32> {
//     match x {
//         None => None,
//         Some(i) => Some(i+1),
//     }
// }



// #[derive(Debug)]
// enum UsState {
//     Alabama,
//     Alaska,
//     Arizona,
//     Arkanas,    
//     // --snip--
// }
    


// enum  Coin {
//     Penny(UsState),
//     Nickel(UsState),
//     Dime(UsState),
//     Quarter(UsState),
// }

// fn value_in_cents(coin:Coin) -> u8 {
//     match coin {
//         Coin::Penny(state) => {
//             match_state(state);
//             1
//         },
//         Coin::Nickel(state) => {
//             match_state(state);
//             5
//         },
//         Coin::Dime(state ) => {
//             match_state(state);
//             10
//         },
//         Coin::Quarter(state) => {
//             match_state(state);
           
//             25
//         },
//     }
// }
// fn match_state(state: UsState) {
//     match state {
//         UsState::Alabama => println!("State quarter from {:?}", state),
//         UsState::Alaska => println!("State quarter from {:?}", state),
//         UsState::Arizona => println!("State quarter from {:?}", state),
//         UsState::Arkanas => println!("State quarter from {:?}", state),
//     }
// }

// enum IpAddrKind {
//     V4(u8, u8, u8, u8),
//     V6(u8, u8 ,u8, u8)
// }
// enum  Message{
//     Quit,
//     Move {x: i32, y: i32},
//     Write(String),
//     ChangeColor(i32, i32, i32)
    
// }
// impl Message {
//     fn some_function(&self) {
//         println!("Hello");
//     }
// }
// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }

// fn main() {
  
//     let localhost = IpAddrKind::V4(127, 0, 0, 1);

// fn route(ip_kind: IpAddrKind) {

// }
// }