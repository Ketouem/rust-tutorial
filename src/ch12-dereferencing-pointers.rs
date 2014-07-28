// Rust uses the unary star operator (*) to access the contents of a box or pointer.

let owned = box 10i;
let borrowed = &20i;

let sum = *owned + *borrowed;

// Dereferenced mutable pointers may appear on the left hand side of assignments. 
// Such an assignment modifies the value that the pointer points to.

let mut owned = box 10i;

let mut value = 20i;
let borrowed = &mut value;

*owned = *borrowed + 100;
*borrowed = *owned + 1000;

/*Pointers have high operator precedence, but lower precedence than the dot operator 
used for field and method access. This precedence order can sometimes make 
code awkward and parenthesis-filled.*/

let start = box Point { x: 10.0, y: 20.0 };
let end = box Point { x: (*start).x + 100.0, y: (*start).y + 100.0 };
let rect = &Rectangle(*start, *end);
let area = (*rect).area();

/*To combat this ugliness the dot operator applies automatic pointer dereferencing 
to the receiver (the value on the left-hand side of the dot), so in most cases, 
explicitly dereferencing the receiver is not necessary.*/

let start = box Point { x: 10.0, y: 20.0 };
let end = box Point { x: start.x + 100.0, y: start.y + 100.0 };
let rect = &Rectangle(*start, *end);
let area = rect.area();

/*You can write an expression that dereferences any number of pointers automatically. 
For example, if you feel inclined, you could write something silly like*/

let point = &box Point { x: 10.0, y: 20.0 };
println!("{:f}", point.x);

// Note: The indexing operator ([]) also auto-dereferences.