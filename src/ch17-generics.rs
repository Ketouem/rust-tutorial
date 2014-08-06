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
