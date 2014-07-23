use std::f64;

fn main() {
	// Struct

	struct Point {
	    x: f64,
	    y: f64
	}

	// Inherited mutability
	let mut mypoint = Point { x: 1.0, y: 1.0 };
	let origin = Point { x: 0.0, y: 0.0 };

	mypoint.y += 1.0; // `mypoint` is mutable, and its fields as well
	//origin.y += 1.0; // ERROR: assigning to immutable field

	// Pattern matching with struct
	match mypoint {
	    Point { x: 0.0, y: yy } => println!("{}", yy),
	    Point { x: xx,  y: yy } => println!("{} {}", xx, yy)
	}
	// To ignore fields use '..'
	match mypoint {
	    Point { x, .. } => println!("{}", x)
	}

	// Enum

	enum Direction {
	    North,
	    East,
	    South,
	    West
	}

	// When an enum has an integral discriminator, cast is quite easy
	println!( "North => {}", North as int );

	// It's possible to use constant as discriminator value
	enum Color {
	  Red = 0xff0000,
	  Green = 0x00ff00,
	  Blue = 0x0000ff
	}

	// Variants can be of a more complex type 
	enum Shape {
	    Circle(Point, f64),
	    Rectangle(Point, Point)
	}
	/* A value of this type is either a Circle or a Rectangle
	 * This declaration defines a type Shape that can refer to such shapes, 
	 * and two functions, Circle and Rectangle, which can be used to 
	 * construct values of the type.
	 */

	// To create a new Circle
	let circle = Circle(Point { x: 0.0, y: 0.0 }, 10.0);

	// All of these variants' constructor can be used as pattern

	fn area(sh: Shape) -> f64 {
	    match sh {
	        Circle(_, size) => f64::consts::PI * size * size,
	        Rectangle(Point { x, y }, Point { x: x2, y: y2 }) => (x2 - x) * (y2 - y)
	    }
	}

	let rect = Rectangle(Point { x: 0.0, y: 0.0 }, Point { x: 2.0, y: 2.0 });
	println!("area: {}", area(rect));

	fn point_from_direction(dir: Direction) -> Point {
	    match dir {
	        North => Point { x:  0.0, y:  1.0 },
	        East  => Point { x:  1.0, y:  0.0 },
	        South => Point { x:  0.0, y: -1.0 },
	        West  => Point { x: -1.0, y:  0.0 }
	    }
	}

	// Tuples
	let mytup: (int, int, f64) = (10, 20, 30.0);
	match mytup {
	  (a, b, c) => println!("{}", a + b + (c as int))
	}

	// Tuple structs, tuple structs have name, fields do not like tuple
	struct MyTup(int, int, f64);
	let mytup: MyTup = MyTup(10, 20, 30.0);
	match mytup {
	  MyTup(a, b, c) => println!("{}", a + b + (c as int))
	}

	// Tuple struct with a single field: newtype
	struct GizmoId(int);
	struct Inches(int);
	struct Centimeters(int);

	let length_with_unit = Inches(10);
	let Inches(integer_length) = length_with_unit;
	println!("length is {} inches", integer_length);
}