use std::io;

fn main() {
	println!("Please enter a number!");

	let mut random = String::new();

	let arr = ["January", "August", "May"];

	io::stdin()
		.read_line(&mut random)
		.expect("Could not read line!");

	let random: usize = random
		.trim()
		.parse()
		.expect("Index entered was not a number!");

	let element = arr[random];

	println!("Value at index {random} is {element}");

}