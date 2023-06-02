use std::io;

fn main() {
    let input = get_user_number();
    let output = fibonacci(input);
    println!("The {}th fibonacci number is {}", input, output);
}

fn fibonacci(n: u128) -> u128 {
    // this is o*n^2
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    } else {
        return fibonacci(n - 1) + fibonacci(n - 2);
    }
}

fn get_user_number() -> u128 {
    loop {
        let mut input = String::new();
        println!("What fibonacci number do you want to see?");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let input: u128 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        return input;
    }
}
