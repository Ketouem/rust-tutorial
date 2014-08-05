/*Methods are like functions except that they always begin with a special argument, called self, 
which has the type of the method's receiver. The self argument is like this in C++ and many other 
languages. Methods are called with dot notation, as in my_vec.len().

Implementations, written with the impl keyword, can define methods on most Rust types, 
including structs and enums. As an example, let's define a draw method on our Shape enum.*/

struct Point {
    x: f64,
    y: f64
}

enum Shape {
    Circle(Point, f64),
    Rectangle(Point, Point)
}

impl Shape {
    fn draw(&self) {
        match *self {
            Circle(p, f) => draw_circle(p, f),
            Rectangle(p1, p2) => draw_rectangle(p1, p2)
        }
    }
}

let s = Circle(Point { x: 1.0, y: 2.0 }, 3.0);
s.draw();

/*This defines an implementation for Shape containing a single method, draw. 
In most respects the draw method is defined like any other function, except for the name self.*/

/*The type of self is the type on which the method is implemented, or a pointer thereof. 
As an argument it is written either self, &self, or self: TYPE. A caller must in turn 
have a compatible pointer type to call the method.*/

impl Shape {
    fn draw_reference(&self) { /* ... */ }
    fn draw_owned(self: Box<Shape>) { /* ... */ }
    fn draw_value(self) { /* ... */ }
}

let s = Circle(Point { x: 1.0, y: 2.0 }, 3.0);

(&s).draw_reference();
(box s).draw_owned();
s.draw_value();

/*Methods typically take a reference self type, so the compiler will go to great lengths 
to convert a callee to a reference.*/

// As with typical function arguments, owned pointers
// are automatically converted to references

(box s).draw_reference();

// Unlike typical function arguments, the self value will
// automatically be referenced ...
s.draw_reference();

// ... and dereferenced
(& &s).draw_reference();

// ... and dereferenced and borrowed
(&box s).draw_reference();

/*Implementations may also define standalone (sometimes called "static") methods. The absence of 
a self parameter distinguishes such methods. These methods are the preferred way to 
define constructor functions.*/

impl Circle {
    fn area(&self) -> f64 { /* ... */ }
    fn new(area: f64) -> Circle { /* ... */ }
}

//To call such a method, just prefix it with the type name and a double colon:

use std::f64::consts::PI;
struct Circle { radius: f64 }
impl Circle {
    fn new(area: f64) -> Circle { Circle { radius: (area / PI).sqrt() } }
}
let c = Circle::new(42.5);