fn main() {
	// this is how arrays are init. in .rs
	// Rust arrays must contain only one data type

	let arr = [1, 3, 5]; // without type annotations

	let arr_2: [i32; 5] = [1, 2, 3, 4, 5]; // with type annotations

	// let arr_2: [3; 5]; apparently haiwezekani

	println!("{}", arr_2[4]); // access the element inindex 4 of arr_3

}