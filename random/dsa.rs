fn main() {
	let tup: (i32, bool, char) = (0, false, 'l'); // define a tuple
	println!("{}", tup.2);

	let mut arr: [i32; 5] = [1, 2, 3, 4, 5]; // define an array with 5 int elements
	println!("{}", arr[4]);
}