fn main() {
	// Linked list implemenation, the use of Box allows for dynamic memory allocation
	// the same version without the Box would cause a compilation error
	enum List {
    	Cons(u32, Box<List>),
    	Nil
	}

	// An instance of the list would look like
	let list = Cons(1, box Cons(2, box Cons(3, box Nil)));

	let xs = Cons(1, box Cons(2, box Cons(3, box Nil)));
	let ys = xs; // copies `Cons(u32, pointer)` shallowly

	let mut xs = Nil;
	let ys = xs;

	// attempting to use `xs` will result in an error here
	xs = Nil;
	// `xs` can be used again

	// Avoiding a move can be done with a clone
	let x = box 5i;
	let y = x.clone(); // `y` is a newly allocated box
	let z = x; // no new memory allocated, `x` can no longer be used

	// We can use clone for the List by deriving from the Clone trait
	#[deriving(Clone)]
	enum List {
	    Cons(u32, Box<List>),
	    Nil
	}

	let x = Cons(5, box Nil);
	let y = x.clone();
	// `x` can still be used!

	let z = x;
	// and now, it can no longer be used since it has been moved

	fn prepend(xs: List, value: u32) -> List {
	    Cons(value, box xs)
	}
	// Not very flexible as it requires ownership vs mutating in place

	let mut xs = Nil;
	xs = prepend(xs, 1);
	xs = prepend(xs, 2);
	xs = prepend(xs, 3);

	// References

	// Implementing an equality comparison without taking ownership of the list
	// by passing the list by references
	fn eq(xs: &List, ys: &List) -> bool {
	    // Match on the next node in both lists.
	    match (xs, ys) {
	        // If we have reached the end of both lists, they are equal.
	        (&Nil, &Nil) => true,
	        // If the current elements of both lists are equal, keep going.
	        // ref keyword can be used to bind to a variable name by-reference rather than by-value
	        (&Cons(x, box ref next_xs), &Cons(y, box ref next_ys))
	                if x == y => eq(next_xs, next_ys),
	        // If the current elements are not equal, the lists are not equal.
	        _ => false
	    }
	}
}