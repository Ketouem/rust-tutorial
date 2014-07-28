/*In contrast with owned boxes, where the holder of an owned box is the owner of the pointed-to memory, 
 *references never imply ownership - they are "borrowed". You can borrow a reference to any object, 
 *and the compiler verifies that it cannot outlive the lifetime of the object.*/

struct Point {
    x: f64,
    y: f64
}

// Differente way to allocate a point at different location
let on_the_stack :     Point  =     Point { x: 3.0, y: 4.0 };
let on_the_heap  : Box<Point> = box Point { x: 7.0, y: 9.0 };

// Let's say we want to create a function that computes the distance between two points
// In order to avoid unecessay copy, we'll pass point using ref

fn compute_distance(p1: &Point, p2: &Point) -> f64 {
    let x_d = p1.x - p2.x;
    let y_d = p1.y - p2.y;
    (x_d * x_d + y_d * y_d).sqrt()
}

// We can call this function on our previously defined points

compute_distance(&on_the_stack, &*on_the_heap);

/*Here the & operator is used to take the address of the variable on_the_stack; 
 *this is because on_the_stack has the type Point (that is, a struct value) and 
 *we have to take its address to get a reference. We also call this borrowing the 
 *local variable on_the_stack, because we are creating an alias: that is, 
 *another route to the same data.
 *Likewise, in the case of owned_box, the & operator is used in conjunction with 
 *the * operator to take a reference to the contents of the box.*/

// Freezing

/*Lending an &-pointer to an object freezes the pointed-to object and prevents 
mutation—even if the object was declared as mut.  Freeze objects have freezing 
enforced statically at compile-time. An example of a non-Freeze type is RefCell<T>.*/

let mut x = 5i;
{
    let y = &x; // `x` is now frozen. It cannot be modified or re-assigned.
}
// `x` is now unfrozen again