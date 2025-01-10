//Here is a program to test your functions.

use capitalizing::*;

fn main() {
    println!("{}", capitalize_first("joe is missing"));
    println!("{}", title_case("jill is leaving A"));
    println!("{}",change_case("heLLo THere"));
}

//And its output
//
//$ cargo run
//Joe is missing
//Jill Is Leaving A
//HEllO thERE
//$
