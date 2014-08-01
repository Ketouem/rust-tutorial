/*Methods are like functions except that they always begin with a special argument, called self, 
which has the type of the method's receiver. The self argument is like this in C++ and many other 
anguages. Methods are called with dot notation, as in my_vec.len().

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