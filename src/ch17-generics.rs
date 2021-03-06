/*With type parameters we can also define functions whose arguments have generic types, 
and which can be invoked with a variety of types. Consider a generic map function, 
which takes a function function and a vector vector and returns a new vector consisting 
of the result of applying function to each element of vector.*/

fn map<T, U>(vector: &[T], function: |v: &T| -> U) -> Vec<U> {
    let mut accumulator = Vec::new();
    for element in vector.iter() {
        accumulator.push(function(element));
    }
    return accumulator;
}

/*When defined with type parameters, as denoted by <T, U>, this function can be applied 
to any type of vector, as long as the type of function's argument and the type of the 
vector's contents agree with each other.*/

/*Inside a generic function, the names of the type parameters (capitalized by convention) 
stand for opaque types. All you can do with instances of these types is pass them around: 
you can't apply any operations to them or pattern-match on them. Note that instances of 
generic types are often passed by pointer.*/

Generic type, struct, and enum declarations follow the same pattern:

type Set<T> = std::collections::HashMap<T, ()>;

struct Stack<T> {
    elements: Vec<T>
}

enum Option<T> {
    Some(T),
    None
}

/*These declarations can be instantiated to valid types like Set<int>, Stack<int>, and Option<int>.*/

/*The last type in that example, Option, appears frequently in Rust code. Because Rust does not have 
null pointers (except in unsafe code), we need another way to write a function whose result isn't 
defined on every possible combination of arguments of the appropriate types. The usual way is to write 
a function that returns Option<T> instead of T.*/

fn radius(shape: Shape) -> Option<f64> {
    match shape {
        Circle(_, radius) => Some(radius),
        Rectangle(..)     => None
    }
}

/*The Rust compiler compiles generic functions very efficiently by monomorphizing them. Monomorphization 
is a fancy name for a simple idea: generate a separate copy of each generic function at each call site, 
a copy that is specialized to the argument types and can thus be optimized specifically for them.*/

// -- Traits --

/*Traits are Rust's most powerful tool for writing polymorphic code. Java developers will see them as 
similar to Java interfaces, and Haskellers will notice their similarities to type classes. Rust's traits 
give us a way to express bounded polymorphism: by limiting the set of possible types that a type parameter 
could refer to, they expand the number of operations we can safely perform on arguments of that type.

Ex: The clone method is not defined for values of every type. One reason is user-defined destructors: 
copying a value of a type that has a destructor could result in the destructor running multiple times. 
Therefore, values of types that have destructors cannot be copied unless we explicitly implement clone for them.

This complicates handling of generic functions. If we have a function with a type parameter T, can we copy values 
of type T inside that function? In Rust, we can't, and if we try to run the following code the compiler will complain.*/

// This does not compile
fn head_bad<T>(v: &[T]) -> T {
    v[0] // error: copying a non-copyable value
}

/*However, we can tell the compiler that the head function is only for copyable types. In Rust, copyable types 
are those that implement the Clone trait. We can then explicitly create a second copy of the value we are 
returning by calling the clone method:*/

// This does
fn head<T: Clone>(v: &[T]) -> T {
    v[0].clone()
}

/*The bounded type parameter T: Clone says that head can be called on an argument of type &[T] for any T, so long 
as there is an implementation of the Clone trait for T. When instantiating a generic function, we can only instantiate 
it with types that implement the correct trait, so we could not apply head to a vector whose elements are of some type 
that does not implement Clone.

While most traits can be defined and implemented by user code, three traits are automatically derived and implemented 
for all applicable types by the compiler, and may not be overridden:

-- Send - Sendable types. Types are sendable unless they contain references.

-- Share - Types that are threadsafe. These are types that are safe to be used across several threads with 
   access to a &T pointer. Mutex<T> is an example of a sharable type with internal mutable data.

-- 'static - Non-borrowed types. These are types that do not contain any data whose lifetime is bound to a 
   particular stack frame. These are types that do not contain any references, or types where the only contained 
   references have the 'static lifetime.

These 3 traits are aka 'kinds'.

Additionally, the Drop trait is used to define destructors. This trait provides one method called drop, 
which is automatically called when a value of the type that implements this trait is destroyed, either because the value 
went out of scope or because the garbage collector reclaimed it.*/

struct TimeBomb {
    explosivity: uint
}

impl Drop for TimeBomb {
    fn drop(&mut self) {
        for _ in range(0, self.explosivity) {
            println!("blam!");
        }
    }
}

// -- Declaring and implementing traits --

/* At its simplest, a trait is a set of zero or more method signatures. For example, we could declare the trait Printable 
for things that can be printed to the console, with a single method signature:*/

trait Printable {
    fn print(&self);
}

/* We say that the Printable trait provides a print method with the given signature. This means that we can call print on an 
argument of any type that implements the Printable trait.

Traits may be implemented for specific types with impls. An impl for a particular trait gives an implementation of the methods 
that trait provides. For instance, the following impls of Printable for int and String give implementations of the print method.*/

impl Printable for int {
    fn print(&self) { println!("{}", *self) }
}

impl Printable for String {
    fn print(&self) { println!("{}", *self) }
}

// Methods defined in an impl for a trait may be called just like any other method, using dot notation, as in 1.print().

// --  Default method implementations in trait definitions --

/* Sometimes, a method that a trait provides will have the same implementation for most or all of the types that implement that 
trait. For instance, suppose that we wanted bools and f32s to be printable, and that we wanted the implementation of print for those 
types to be exactly as it is for int, above:*/

impl Printable for f32 {
    fn print(&self) { println!("{}", *self) }
}

impl Printable for bool {
    fn print(&self) { println!("{}", *self) }
}

/* This works fine, but we've now repeated the same definition of print in three places. Instead of doing that, we can simply 
include the definition of print right in the trait definition, instead of just giving its signature. That is, we can write the following:*/

extern crate debug;

trait Printable {
    // Default method implementation
    fn print(&self) { println!("{:?}", *self) }
}

impl Printable for int {}

impl Printable for String {
    fn print(&self) { println!("{}", *self) }
}

impl Printable for bool {}

impl Printable for f32 {}

/* Here, the impls of Printable for int, bool, and f32 don't need to provide an implementation of print, because in the absence of a 
specific implementation, Rust just uses the default method provided in the trait definition.*/

// -- Type-parameterized traits --

/* Traits may be parameterized by type variables. For example, a trait for generalized sequence types might look like the following:*/

trait Seq<T> {
    fn length(&self) -> uint;
}

impl<T> Seq<T> for Vec<T> {
    fn length(&self) -> uint { self.len() }
}

/* The implementation has to explicitly declare the type parameter that it binds, T, before using it to specify its trait type. 
Rust requires this declaration because the impl could also, for example, specify an implementation of Seq<int>. 
The trait type (appearing between impl and for) refers to a type, rather than defining one.

The type parameters bound by a trait are in scope in each of the method declarations. So, re-declaring the type parameter T as an 
explicit type parameter for length, in either the trait or the impl, would be a compile-time error.

Within a trait definition, Self is a special type that you can think of as a type parameter. An implementation of the trait for any 
given type T replaces the Self type parameter with T. The following trait describes types that support an equality operation:*/

// In a trait, `self` refers to the self argument.
// `Self` refers to the type implementing the trait.
trait PartialEq {
    fn equals(&self, other: &Self) -> bool;
}

// In an impl, `self` refers just to the value of the receiver
impl PartialEq for int {
    fn equals(&self, other: &int) -> bool { *other == *self }
}

/* In the trait definition, equals takes a second parameter of type Self. In contrast, in the impl, equals takes a second parameter of type int, 
only using self as the name of the receiver.*/

/* Just as in type implementations, traits can define standalone (static) methods. These methods are called by prefixing the method name with the 
trait name and a double colon. The compiler uses type inference to decide which implementation to use.*/

use std::f64::consts::PI;
trait Shape { fn new(area: f64) -> Self; }
struct Circle { radius: f64 }
struct Square { length: f64 }

impl Shape for Circle {
    fn new(area: f64) -> Circle { Circle { radius: (area / PI).sqrt() } }
}
impl Shape for Square {
    fn new(area: f64) -> Square { Square { length: area.sqrt() } }
}

let area = 42.5;
let c: Circle = Shape::new(area);
let s: Square = Shape::new(area);

// -- Bounded type parameters and static method dispatch --

/* Traits give a language for defining predicates on types, or abstract properties that types can have. We can use this language to define bounds 
on type parameters, so that we can then operate on generic types.*/

fn print_all<T: Printable>(printable_things: Vec<T>) {
    for thing in printable_things.iter() {
        thing.print();
    }
}

/* Declaring T as conforming to the Printable trait (as we earlier did with Clone) makes it possible to call methods from that trait on values of 
type T inside the function. It will also cause a compile-time error when anyone tries to call print_all on a vector whose element type 
does not have a Printable implementation.

Type parameters can have multiple bounds by separating them with +, as in this version of print_all that copies elements.*/

fn print_all<T: Printable + Clone>(printable_things: Vec<T>) {
    let mut i = 0;
    while i < printable_things.len() {
        let copy_of_thing = printable_things[i].clone();
        copy_of_thing.print();
        i += 1;
    }
}

/* Method calls to bounded type parameters are statically dispatched, imposing no more overhead than normal function invocation, so are the preferred 
way to use traits polymorphically.*/

// -- Trait objects and dynamic method dispatch --

/*The above allows us to define functions that polymorphically act on values of a single unknown type that conforms to a given trait. 
However, consider this function:*/

trait Drawable { fn draw(&self); }

fn draw_all<T: Drawable>(shapes: Vec<T>) {
    for shape in shapes.iter() { shape.draw(); }
}

/*You can call that on a vector of circles, or a vector of rectangles (assuming those have suitable Drawable traits defined), 
but not on a vector containing both circles and rectangles. When such behavior is needed, 
a trait name can alternately be used as a type, called an object.*/

fn draw_all(shapes: &[Box<Drawable>]) {
    for shape in shapes.iter() { shape.draw(); }
}

/*In this example, there is no type parameter. Instead, the Box<Drawable> type denotes any owned box value that implements the Drawable trait. 
To construct such a value, you use the as operator to cast a value to an object:*/


impl Drawable for Circle { fn draw(&self) { /* ... */ } }
impl Drawable for Rectangle { fn draw(&self) { /* ... */ } }

let c: Box<Circle> = box new_circle();
let r: Box<Rectangle> = box new_rectangle();
draw_all([c as Box<Drawable>, r as Box<Drawable>]);

/*Note that, like strings and vectors, objects have dynamic size and may only be referred to via one of the pointer types. Other pointer types work as well. 
Casts to traits may only be done with compatible pointers so, for example, an &Circle may not be cast to a Box<Drawable>.*/

// An owned object
let owny: Box<Drawable> = box new_circle() as Box<Drawable>;
// A borrowed object
let stacky: &Drawable = &new_circle() as &Drawable;

/*Method calls to trait types are dynamically dispatched. Since the compiler doesn't know specifically which functions to call at compile time, 
it uses a lookup table (also known as a vtable or dictionary) to select the method to call at runtime.

This usage of traits is similar to Java interfaces.

There are some built-in bounds, such as Send and Sync, which are properties of the components of types. By design, trait objects don't know 
the exact type of their contents and so the compiler cannot reason about those properties.

You can instruct the compiler, however, that the contents of a trait object must ascribe to a particular bound with a trailing colon (:). 
These are examples of valid types:*/

trait Foo {}
trait Bar<T> {}

fn sendable_foo(f: Box<Foo + Send>) { /* ... */ }
fn sync_bar<T: Sync>(b: &Bar<T> + Sync) { /* ... */ }

/*When no colon is specified (such as the type Box<Foo>), it is inferred that the value ascribes to no bounds. They must be added manually 
if any bounds are necessary for usage.

Builtin kind bounds can also be specified on closure types in the same way (for example, by writing fn:Send()), and the default behaviours are the same as 
for traits of the same storage class.*/

// -- Trait inheritance --

/*We can write a trait declaration that inherits from other traits, called supertraits. Types that implement a trait must also implement its supertraits. 
For example, we can define a Circle trait that inherits from Shape.*/

trait Shape { fn area(&self) -> f64; }
trait Circle : Shape { fn radius(&self) -> f64; }

// Now, we can implement Circle on a type only if we also implement Shape.

use std::f64::consts::PI;
struct CircleStruct { center: Point, radius: f64 }
impl Circle for CircleStruct {
    fn radius(&self) -> f64 { (self.area() / PI).sqrt() }
}
impl Shape for CircleStruct {
    fn area(&self) -> f64 { PI * square(self.radius) }
}

/*Notice that methods of Circle can call methods on Shape, as our radius implementation calls the area method.
In type-parameterized functions, methods of the supertrait may be called on values of subtrait-bound type parameters. 
Referring to the previous example of trait Circle : Shape:*/

fn radius_times_area<T: Circle>(c: T) -> f64 {
    // `c` is both a Circle and a Shape
    c.radius() * c.area()
}

// Likewise, supertrait methods may also be called on trait objects.

use std::f64::consts::PI;

let concrete = box CircleStruct{center:Point{x:3.0,y:4.0},radius:5.0};
let mycircle: Box<Circle> = concrete as Box<Circle>;
let nonsense = mycircle.radius() * mycircle.area();

// -- Deriving implementations for traits --

/*A small number of traits in can have implementations that can be automatically derived. These instances are specified by placing the deriving 
attribute on a data type declaration. For example, the following will mean that Circle has an implementation for PartialEq and can be used 
with the equality operators, and that a value of type ABC can be randomly generated and converted to a string:*/

extern crate rand;
use std::rand::{task_rng, Rng};

#[deriving(PartialEq)]
struct Circle { radius: f64 }

#[deriving(Rand, Show)]
enum ABC { A, B, C }

fn main() {
    // Use the Show trait to print "A, B, C."
    println!("{}, {}, {}", A, B, C);

    let mut rng = task_rng();

    // Use the Rand trait to generate a random variants.
    for _ in range(0i, 10) {
        println!("{}", rng.gen::<ABC>());
    }
}

// The full list of derivable traits is PartialEq, Eq, PartialOrd, Ord, Encodable, Decodable, Clone, Hash, Rand, Default, Zero, FromPrimitive and Show.