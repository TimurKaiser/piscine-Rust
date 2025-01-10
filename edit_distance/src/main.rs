//Here is a program to test your function.

use edit_distance::*;

fn main() {
	let source = "alignment";
	let target = "assignment";
	println!(
		"It's necessary to make {} change(s) to {}, to get {}",
		edit_distance(source, target),
		source,
		target
	);
}

//And its output:
//
//$ cargo run
//It's necessary to make 2 change(s) to alignment, to get assignment
//$
