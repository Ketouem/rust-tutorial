/*Named functions, like those we've seen so far, may not refer to local variables declared outside the function: 
they do not close over their environment (sometimes referred to as "capturing" variables in their environment).
A closure does support accessing the enclosing scope
*/

let x = 3;

// `fun` is an invalid definition
fn  fun       () -> () { println!("{}", x) }  // cannot capture from enclosing scope
let closure = || -> () { println!("{}", x) }; // can capture from enclosing scope

// `fun_arg` is an invalid definition
fn  fun_arg       (arg: int) -> () { println!("{}", arg + x) }  // cannot capture
let closure_arg = |arg: int| -> () { println!("{}", arg + x) }; // can capture
//                      ^
// Requires a type because the implementation needs to know which `+` to use.
// In the future, the implementation may not need the help.

fun();          // Still won't work
closure();      // Prints: 3

fun_arg(7);     // Still won't work
closure_arg(7); // Prints: 10

/*Closures begin with the argument list between vertical bars and are followed by a single expression. 
Remember that a block, { <expr1>; <expr2>; ... }, is considered a single expression: 
it evaluates to the result of the last expression it contains if that expression is not followed 
by a semicolon, otherwise the block evaluates to (), the unit value.

In general, return types and all argument types must be specified explicitly for function definitions.*/

fn  fun   (x: int)         { println!("{}", x) } // this is same as saying `-> ()`
fn  square(x: int) -> uint { (x * x) as uint }   // other return types are explicit

// Error: mismatched types: expected `()` but found `uint`
fn  badfun(x: int)         { (x * x) as uint }

/*On the other hand, the compiler can usually infer both the argument and return types for a 
closure expression; therefore they are often omitted, since both a human reader and the compiler 
can deduce the types from the immediate context. This is in contrast to function declarations, 
which require types to be specified and are not subject to type inference.*/

// `fun` as a function declaration cannot infer the type of `x`, so it must be provided
fn  fun       (x: int) { println!("{}", x) }
let closure = |x     | { println!("{}", x) }; // infers `x: int`, return type `()`

// For closures, omitting a return type is *not* synonymous with `-> ()`
let add_3   = |y     | { 3i + y }; // infers `y: int`, return type `int`.

fun(10);            // Prints 10
closure(20);        // Prints 20
closure(add_3(30)); // Prints 33

fun("String"); // Error: mismatched types

// Error: mismatched types
// inference already assigned `closure` the type `|int| -> ()`
closure("String");

/*In cases where the compiler needs assistance, the arguments and return types may be annotated 
on closures, using the same notation as shown earlier. In the example below, since different types 
provide an implementation for the operator *, the argument type for the x parameter must be explicitly provided.*/

// Error: the type of `x` must be known to be used with `x * x`
let square = |x     | -> uint { (x * x) as uint };

let square_explicit = |x: int| -> uint { (x * x) as uint };
let square_infer    = |x: int|         { (x * x) as uint };

println!("{}", square_explicit(20));  // 400
println!("{}", square_infer(-20));    // 400

/*There are several forms of closure, each with its own role. The most common, called a stack closure, 
has type || and can directly access local variables in the enclosing scope.*/

let mut max = 0;
let f = |x: int| if x > max { max = x };
for x in [1, 2, 3].iter() {
    f(*x);
}

/*Stack closures are very efficient because their environment is allocated on the call stack and refers by 
pointer to captured locals. To ensure that stack closures never outlive the local variables to which they refer, 
stack closures are not first-class. That is, they can only be used in argument position; 
they cannot be stored in data structures or returned from functions. 
Despite these limitations, stack closures are used pervasively in Rust code.*/

// -- Owned closures

/*Owned closures, written proc, hold on to things that can safely be sent between processes. They copy the values they close over, 
but they also own them: that is, no other code can access them. Owned closures are used in concurrent code, particularly for spawning tasks.
Closures can be used to spawn tasks. A practical example of this pattern is found when using the spawn function, which starts a new task.*/

use std::task::spawn;

// proc is the closure which will be spawned.
spawn(proc() {
    println!("I'm a new task")
});

// -- Closure compatibility

/*Rust closures have a convenient subtyping property: you can pass any kind of closure (as long as the arguments and return types match) 
to functions that expect a ||. Thus, when writing a higher-order function that only calls its function argument, and does nothing else with it, 
you should almost always declare the type of that argument as ||. That way, callers may pass any kind of closure.*/

fn call_twice(f: ||) { f(); f(); }
let closure = || { "I'm a closure, and it doesn't matter what type I am"; };
fn function() { "I'm a normal function"; }
call_twice(closure);
call_twice(function);