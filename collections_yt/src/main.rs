
use std::collections::HashMap;
fn main() {
    // let blue = String::from("blue");
    // let yellow = String::from("yellow");
    // let mut scores = HashMap::new();
    // scores.insert(blue, 10);
    // scores.insert(yellow, 50);

    // let  team_name = String::from("blue");
    // let score = scores.get(&team_name);
    // let mut scores = HashMap::new();
    // scores.insert(String::from("blue"), 10);
    // scores.insert(String::from("yellow"), 50);
    // scores.entry(String::from("blue")).or_insert(50);
    // scores.entry(String::from("yellow")).or_insert(50);

    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0); // returns a mutable reference (&mut V) to the value for this key
        *count += 1;
    }
    println!("{:?}", map);

//    let a = [1,2,3];
//    let mut v: Vec<i32> = Vec::new();
//     v.push(5);
//     v.push(6);
//     v.push(7);
//     let mut v2 = vec![1,2,3,4,5];
//     let _third: &i32 = &v2[2];
//     // println!("The third element is {}", third);

//     // match v2.get(2) {
//     //     Some(third) => println!("The third element is {}", third),
//     //     None => println!("There is no third element."),
//     // }
//     for i in &mut v2 {
//       *i += 50;
      
   
//     }
//     for i in &v2 {
    
//         println!("{}", i);
//     }
    // enum SpreadsheetCell {
    //     Int(i32),
    //     Float(f64),
    //     Text(String),  
    // }

    // let row = vec![
    //     SpreadsheetCell::Int(3),
    //     SpreadsheetCell::Text(String::from("blue")),
    //     SpreadsheetCell::Float(10.12),
    // ];

    // for i in &row {
    //     match i {
    //         SpreadsheetCell::Int(i) => println!("This value is int {}", i),
    //         SpreadsheetCell::Float(f) => println!( "this value is float {}", f),
    //         SpreadsheetCell::Text(s) => println!("this value is a string {}", s),
    //     }
    // }
    
//     let hello = String::from( "Здравствуйте");

// // bytes
//     for b in hello.bytes() {
//         println!("{}", b);


//     }
//     // Scalar values
//     for c in hello.chars() {
//         println!("{}", c);
//     }
    
//     // Graphemes Require external crate
//     for e in hello.graphemes(true) {
//         println!("{}", e);
//     }


}
