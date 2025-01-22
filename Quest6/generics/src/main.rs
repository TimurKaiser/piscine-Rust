//Here is a program to test your function.

use generics::*;

fn main() {
	println!("{}", identity("Hello, world!"));
	println!("{}", identity(3));
}

And its output:

//$ cargo run
//Hello, world!
//3
//$
