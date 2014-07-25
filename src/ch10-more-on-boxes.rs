// The most common use case for owned boxes is creating recursive data structures like a binary search tree. 

/*Note that returning large unboxed values via boxes is unnecessary. A large value is returned via a hidden output parameter, 
and the decision on where to place the return value should be left to the caller:*/

fn foo() -> (u64, u64, u64, u64, u64, u64) {
    (5, 5, 5, 5, 5, 5)
}

let x = box foo(); // allocates a box, and writes the integers directly to it

/*Beyond the properties granted by the size, an owned box behaves as a regular value by inheriting 
the mutability and lifetime of the owner:*/

let x = 5i; // immutable
let mut y = 5i; // mutable
y += 2;

let x = box 5i; // immutable
let mut y = box 5i; // mutable
*y += 2; // the `*` operator is needed to access the contained value