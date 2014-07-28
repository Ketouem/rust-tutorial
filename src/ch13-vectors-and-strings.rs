/*A vector is a contiguous block of memory containing zero or more values of the same type. 
Rust also supports vector reference types, called slices, 
which are a view into a block of memory represented as a pointer and a length.

Strings are represented as vectors of u8, with the guarantee of containing 
a valid UTF-8 sequence.

Fixed-size vectors are an unboxed block of memory, with the element length as part 
of the type. A fixed-size vector owns the elements it contains, so the elements 
are mutable if the vector is mutable. Fixed-size strings do not exist.*/

// -- A fixed-size vector
let numbers = [1i, 2, 3];
let more_numbers = numbers;

// The type of a fixed-size vector is written as `[Type, ..length]`
let five_zeroes: [int, ..5] = [0, ..5];

/*A unique vector is dynamically sized, and has a destructor to clean up allocated 
memory on the heap. A unique vector owns the elements it contains, 
so the elements are mutable if the vector is mutable.*/

use std::string::String;

// -- A dynamically sized vector (unique vector)
let mut numbers = vec![1i, 2, 3];
numbers.push(4);
numbers.push(5);

// The type of a unique vector is written as `Vec<int>`
let more_numbers: Vec<int> = numbers.move_iter().map(|i| i+1).collect();

// The original `numbers` value can no longer be used, due to move semantics.

let mut string = String::from_str("fo");
string.push_char('o');

// Slices are similar to fixed-size vectors, but the length is not part of the type. 
// They simply point into a block of memory and do not have ownership over the elements.

// A slice
let xs = &[1, 2, 3];

// Slices have their type written as `&[int]`
let ys: &[int] = xs;

// Other vector types coerce to slices
let three = [1, 2, 3];
let zs: &[int] = three;

// An unadorned string literal is an immutable string slice
let string = "foobar";

// A string slice type is written as `&str`
let view: &str = string.slice(0, 3);

// Square brackets denote indexing into a slice or fixed-size vector:

let crayons: [&str, ..3] = ["BananaMania", "Beaver", "Bittersweet"];
println!("Crayon 2 is '{}'", crayons[2]);

/*Mutable slices also exist, just as there are mutable references. However, there are 
no mutable string slices. Strings are a multi-byte encoding (UTF-8) of Unicode code 
points, so they cannot be freely mutated without the ability to alter the length.*/

let mut xs = [1i, 2i, 3i];
let view = xs.mut_slice(0, 2);
view[0] = 5;

// The type of a mutable slice is written as `&mut [T]`
let ys: &mut [int] = &mut [1i, 2i, 3i];

// A slice or fixed-size vector can be destructured using pattern matching:

let numbers: &[int] = &[1, 2, 3];
let score = match numbers {
    [] => 0,
    [a] => a * 10,
    [a, b] => a * 6 + b * 4,
    [a, b, c, ..rest] => a * 5 + b * 3 + c * 2 + rest.len() as int
};

// Note: Both vectors and strings support a number of useful methods, 
// defined in std::vec, std::slice, and std::str.