//Here is a program to test your function.

use string_permutation::*;

fn main() {
	let word = "thought";
	let word1 = "thougth";
	println!(
		"Is `{}` a permutation of `{}`? = {}",
		word,
		word1,
		is_permutation(word, word1)
	);
}

//And its output
//
//$ cargo run
//Is `thought` a permutation of `thougth`? = true
//$

