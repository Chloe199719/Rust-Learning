use std::io;

fn main() {
    // let mut input = String::new();
    // println!("Enter the Temperature Number:");
    // io::stdin().read_line(&mut input).expect("Failed to read line");
    // let input: f32 =  input.trim().parse().expect("Please type a number!");


    loop {
        let input = get_input();
        let mut unit = String::new();
        println!("Enter the Temperature Unit: (C/F)");
        io::stdin().read_line(&mut unit).expect("Failed to read line");
        if unit.trim().to_uppercase() == "C" {
            
            let output = (input * 9.0/5.0) + 32.0;
            println!("{} C = {} F", input, output);
            break;
    }   else if unit.trim().to_uppercase() == "F" {
           
            let output = (input - 32.0) * 5.0/9.0;
            println!("{} F = {} C", input, output);
            break;
    }   else {
            println!("Please enter a valid unit!");
    }
  }




   
}

fn get_input () -> f32 {
 loop {
    let mut input = String::new();
    println!("Enter the Temperature Number:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let input: f32 = match  input.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
    };
    return input;
 }
}