fn main() {
	let x: isize = 5368709120;

	let x = x + 1;

	let t: bool = false;  // explicit type ann. for booleans

	{
		let x = x * 2;
		println!("The value of x in the inner scope is: {x}");
	}

	println!("The value of x is: {x}");
}
