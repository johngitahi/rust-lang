// An attribute to hide warnings for unused code
#![allow(dead_code)]

#[derive(Debug)]
struct Person {
	name: String,
	age: u8,
}

// A unit struct
struct Unit;

// A tuple struct
struct Pair(i32, f32);


// A struct with two fields
struct Point {
	x: f32,
	y: f32,
}

// Structs can be reused as fields of another struct
struct Rectangle {
	// A rectangle can be specified by where the top left
	// and bottom right corners are in space.
	top_left: Point,
	bottom_right: Point,
}

fn main() {
	// Create struct with field init shorthand
	let name = String::from("John");
	let age = 99;
	let john = Person { name, age };

	// print debug struct
	println!("{:?}", john);

	// instantiate a `Point`
	let point: Point = Point { x: 0.0, y: 0.0 };

	// Access the fields of the point
	println!("point coordinates: ({}, {})", point.x, point.y);

	// Make a new point by using struct update syntax to use the 
	// fields of our other one
	let bottom_right = Point { x: 5.2, ..point };

	// `bottom_right.y` will be the same as `point.y` because we
	// used that field from `point`
	println!("Second point: ({}, {})", bottom_right.x, bottom_right.y);

	// Destructure the point using a `let` binding
	let Point { x: left_edge, y: top_edge } = point;

	let _rectangle = Rectangle {
		// struct instantiation is an expression too
		top_left: Point { x: left_edge, y: top_edge },
		bottom_right: bottom_right,
	};

	// Instantiate a unit struct
	let _unit = Unit;


	// Instantiate a tuple struct
	let pair = Pair(1, 0.1);

	// Access the fields of a tuple struct
	println!("pair contains {:?} and {:?}", pair.0, pair.1);

	// Destructure a tuple struct
	let Pair(integer, decimal) = pair;
	let area = rect_area(_rectangle);
	println!("Area {}", area);

	println!("pair contains {:?} and {:?}", integer, decimal);
}

fn rect_area(rectangle: Rectangle) -> f32 {
	// destructure the points from the rectangle
	// let Rectangle{ top_left, bottom_right } = rectangle;
	absolute_length = rectangle.top_left.x - rectangle.bottom_right.x;
	absolute_breadth = rectangle.top_left.y - rectangle.bottom_right.y;

	return absolute_length * absolute_breadth;
}
