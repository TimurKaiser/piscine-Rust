//Here is a program to test your function.

//   It's incomplete. Use the output and the other available information to figure out what is missing.


//use arrays::{sum, thirtytwo_tens};
//
//fn main() {
//	let a = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
//	let a1: Vec<i32> = (1..11).; //missing info here
//	let b = [_; 10]; //missing info here
//
//	println!("The Sum of the elements in {:?} = {}", a, sum(a));//missing info here
//	println!("The Sum of the elements in {:?} = ", a1, sum(a1));//missing info here
//	println!("The Sum of the elements in {:?} = {}", b, sum(b));//missing info here
//	println!(
//		"Array size {} with only 10's in it {:?}",
//		thirtytwo_tens().len(),
//		thirtytwo_tens()
//	);
//}
//
//And its output:
//
//$ cargo run
//The Sum of the elements in [1, 2, 3, 4, 5, 6, 7, 8, 9, 10] = 55
//The Sum of the elements in [1, 2, 3, 4, 5, 6, 7, 8, 9, 10] = 55
//The Sum of the elements in [5, 5, 5, 5, 5, 5, 5, 5, 5, 5] = 50
//Array size 32 with only 10's in it [10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10]
//$
//



use arrays::{sum, thirtytwo_tens};

fn main() {
    let a = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let a1: Vec<i32> = (1..11).collect();
    let b = [5; 10];

    println!("The Sum of the elements in {:?} = {}", a, sum(&a));
    println!("The Sum of the elements in {:?} = {}", a1, sum(&a1));
    println!("The Sum of the elements in {:?} = {}", b, sum(&b));
    println!(
        "Array size {} with only 10's in it {:?}",
        thirtytwo_tens().len(),
        thirtytwo_tens()
    );
}
