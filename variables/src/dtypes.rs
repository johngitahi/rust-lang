fn main() {
	// definition of data types in Rust
	// floating points
	
	let x = 2.0; //default f64 -- double precision

	let y: f32 = 3.0; //f32 -- single precision

	let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    println!("{:?}", heart_eyed_cat);

    // Defining tuples in Rust
    let tup: (u8, char, f32) = (21, 'K', 3.3); // tuple with type annotation
    let tup_2 = (0, 1, 2);

    // accessing values in the tuple

    let TWENTY_ONE:u8 = tup.0;
    let letter = tup.1;
    let one = tup_2.0;

    println!("{:?}", TWENTY_ONE);
    println!("{}", one)
}