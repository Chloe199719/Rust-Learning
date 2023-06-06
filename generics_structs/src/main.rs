struct Point<T , U> {
    x: T,
    y: U,
}

fn main() {
    let integer = Point { x: 5, y: 10 };
    let _float = Point { x: 1.0, y: 4.0 };
    let _mixed = Point { x: 5, y: 4.0 };

    println!("integer.x = {} , integer.y ={}", integer.x, integer.y);
}
