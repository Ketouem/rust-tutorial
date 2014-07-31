/*Ownership can cleanly describe tree-like data structures, and references provide non-owning pointers. 
However, more flexibility is often desired and Rust provides ways to escape from strict single parent ownership.

Reference counted pointer:

The standard library provides the std::rc::Rc pointer type to express shared ownership over a reference counted 
box. As soon as all of the Rc pointers go out of scope, the box and the contained value are destroyed.*/

use std::rc::Rc;

// A fixed-size array allocated in a reference-counted box
let x = Rc::new([1i, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
let y = x.clone(); // a new owner
let z = x; // this moves `x` into `z`, rather than creating a new owner

assert!(*z == [1i, 2, 3, 4, 5, 6, 7, 8, 9, 10]);

// the variable is mutable, but not the contents of the box
let mut a = Rc::new([10, 9, 8, 7, 6, 5, 4, 3, 2, 1]);
a = z;

/*Garbage collected pointer

A garbage collected pointer is provided via std::gc::Gc, with a task-local garbage collector 
having ownership of the box. It allows the creation of cycles, and the individual Gc pointers do not have a destructor.*/

use std::gc::GC;

// A fixed-size array allocated in a garbage-collected box
let x = box(GC) [1i, 2, 3, 4, 5, 6, 7, 8, 9, 10];
let y = x; // does not perform a move, unlike with `Rc`
let z = x;

assert!(*z == [1i, 2, 3, 4, 5, 6, 7, 8, 9, 10]);