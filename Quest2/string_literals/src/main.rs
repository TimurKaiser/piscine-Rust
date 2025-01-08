//Here is a program to test your function

use string_literals::*;

fn main() {
    println!("{}", is_empty(""));
    println!("{}", is_ascii("rust"));
    println!("{}", contains("rust", "ru"));
    println!("{:?}", split_at("rust", 2));
    println!("{}", find("rust", 'u'));
}

//And its output
//
//$ cargo run
//true
//true
//true
//("ru", "st")
//1
//$
//