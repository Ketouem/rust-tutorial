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