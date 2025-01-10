//Here is a program to test your function.

use to_url::*;

fn main() {
    let s = "Hello, world!";
    println!("'{}' parsed as an URL becomes '{}'", s, to_url(s));
}

//And its output
//
//$ cargo run
//'Hello, world!' parsed as an URL becomes 'Hello,%20world!'
//$
