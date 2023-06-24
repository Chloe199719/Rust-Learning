// struct Point<T> {
//     x: T,
//     y: T
// }
// impl <T> Point<T> {
//     fn x(&self) -> &T {
//         &self.x
//     }
// }

// impl Point<f64> {
  
//     fn y(&self) -> f64 {
//         self.y
//     }
// }
    
struct Point<T, U> {
    x: T,
    y: U
}
impl <T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y
        }
    }
}


fn main() {
    // let p = Point { x: 5, y: 10 };
    // let p1 = Point { x: 5.0, y: 10.0 };
    



    // enum  Option<T> {
    //     Some(T),
    //     None,
    // }
        
    // enum Result<T, E> {
    //     Ok(T),
    //     Err(E),
    // }
    // let p1 = Point { x: 5, y: 10 };
    // let p2 = Point { x: 10.1, y: 4.4 };
    // let p3 = Point { x: 10.1, y: 4 };
    // let number_list = vec![34, 50, 25, 100, 65];
    // let largest = get_largest(number_list);
    // println!("The largest number is {}", largest);

    // let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    // let largest = get_largest(number_list);
    // println!("The largest number is {}", largest);

    // let char_list = vec!['y', 'm', 'a', 'q'];
    // let largest = get_largest(char_list);
    // println!("The largest number is {}", largest);

}

// fn  get_largest<T: PartialOrd + Copy>(list: Vec<T>) -> T {
//     let mut largest = list[0];

//     for number in list {
//         if number > largest {
//             largest = number;
//         }
//     }
//     largest
// }