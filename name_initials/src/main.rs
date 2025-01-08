//Here is a program to test your function:

use name_initials::*;

fn main() {
    let names = vec!["Harry Potter", "Someone Else", "J. L.", "Barack Obama"];
    println!("{:?}", initials(names));
}

//And its output
//
//$ cargo run
//["H. P.", "S. E.", "J. L.", "B. O."]
//$
