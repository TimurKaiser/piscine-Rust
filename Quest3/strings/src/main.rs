//Here is a program to test your function.

use strings::*;

fn main() {
	println!("length of {} = {}", "❤", char_length("❤"));
	println!("length of {} = {}", "形声字", char_length("形聲字"));
	println!("length of {} = {}", "change", char_length("change"));
	println!("length of {} = {}", "😍", char_length("😍"));
}

//And its output
//
//$ cargo run
//length of ❤ = 1
//length of 形声字 = 3
//length of change = 6
//length of 😍 = 1
//$
