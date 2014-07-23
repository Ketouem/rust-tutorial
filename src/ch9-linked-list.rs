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

	// List of generics
	enum List<T> {
	    Cons(T, Box<List<T>>),
	    Nil
	}

	// Updating the prepend definition
	fn prepend<T>(xs: List<T>, value: T) -> List<T> {
    	Cons(value, box xs)
	}

	// Prepending using the type inference
	let mut xs = Nil; // Unknown type! This is a `List<T>`, but `T` can be anything.
	xs = prepend(xs, 10i); // Here the compiler infers `xs`'s type as `List<int>`.
	xs = prepend(xs, 15i);
	xs = prepend(xs, 20i);

	// Which is equivalent to using the following type annotations
	let mut xs: List<int> = Nil::<int>;
	xs = prepend::<int>(xs, 10);
	xs = prepend::<int>(xs, 15);
	xs = prepend::<int>(xs, 20);

/*	Note: In declarations, the language uses Type<T, U, V> to describe a list 
	of type parameters, but expressions use identifier::<T, U, V>, to disambiguate the < operator. */

/*	To implement an eq function for our list that manipulates generics. We can add a trait bound 
	on the PartialEq trait to require that the type implement the == operator. 
	Two more ref annotations need to be added to avoid attempting to move out the element types*/

	fn eq<T: PartialEq>(xs: &List<T>, ys: &List<T>) -> bool {
	    // Match on the next node in both lists.
	    match (xs, ys) {
	        // If we have reached the end of both lists, they are equal.
	        (&Nil, &Nil) => true,
	        // If the current elements of both lists are equal, keep going.
	        (&Cons(ref x, box ref next_xs), &Cons(ref y, box ref next_ys))
	                if x == y => eq(next_xs, next_ys),
	        // If the current elements are not equal, the lists are not equal.
	        _ => false
	    }
	}

	let xs = Cons('c', box Cons('a', box Cons('t', box Nil)));
	let ys = Cons('c', box Cons('a', box Cons('t', box Nil)));
	assert!(eq(&xs, &ys));

	// Implementing the PartialEq trait through our list in order to get the == and != operators
	impl<T: PartialEq> PartialEq for List<T> {
	    fn eq(&self, ys: &List<T>) -> bool {
	        // Match on the next node in both lists.
	        // In a method, the self parameter refers to an instance of the type we're implementing on.
	        match (self, ys) {
	            // If we have reached the end of both lists, they are equal.
	            (&Nil, &Nil) => true,
	            // If the current elements of both lists are equal, keep going.
	            (&Cons(ref x, box ref next_xs), &Cons(ref y, box ref next_ys))
	                    if x == y => next_xs == next_ys,
	            // If the current elements are not equal, the lists are not equal.
	            _ => false
	        }
	    }
	}

	let xs = Cons(5i, box Cons(10i, box Nil));
	let ys = Cons(5i, box Cons(10i, box Nil));
	// The methods below are part of the PartialEq trait,
	// which we implemented on our linked list.
	assert!(xs.eq(&ys));
	assert!(!xs.ne(&ys));

	// The PartialEq trait also allows us to use the shorthand infix operators.
	assert!(xs == ys);    // `xs == ys` is short for `xs.eq(&ys)`
	assert!(!(xs != ys)); // `xs != ys` is short for `xs.ne(&ys)`

}