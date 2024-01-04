///Functions and Closures
//We have been using functions already, including the main() which is the program entry point
//In this section, we are particularly highlighting the fact that functions
//have a type unto themselves and variables of a given function type
//can be declared and passed to another function.
//So, we can have a series of function calls, the output of one becoming the input of
//the next. Herein lies the concept of higher order functions

//As already mentioned, in Rust, functions have their own types.
//Below is an illustration

///Function to add to two signed integers. Returns a signed integer
fn add(a: i32, b: i32) -> i32 {
    a + b
}
//The function type embodied in the above is fn(i32, i32) -> i32.
//Function type is defined by the keyword fn followed 
//by the optional expected parameter types
//and then the optional expected return type.

///Here we define a function name apply that is expected to receive the function type
/// above name f here, along with two other unsigned interger parameters named x and y
/// respectively
fn apply_add(f: fn(i32, i32) -> i32, x: i32, y: i32) -> i32 {
    f(x, y) //a call to the function passed, which in its turn is passed two other parameters
}

pub fn run() {
    let f = add;
    let x = 7;
    let y = 8;
    let z = apply_add(f, x, y);
    println!(
        "The result of applying add function f to {} and {} is {}",
        x, y, z
    );
}

///let's define another function that handles straight line graph formula
///Assuminng that m, c and x have to be passed.
///Here you can use a normal function.
///Below, we have to use array slice as x, otherwise, we will need to specify a size.
fn straight_line_function(m: i32, c: i32, xses: &[i32]) -> Vec<(i32, i32)> {
    let mut output: Vec<(i32, i32)> = Vec::new(); //you could also use vec![] to bring in initial arguments
    for x in xses {
        let y = (m * x) + c;
        output.push((*x, y)) //here we have to dereference the borrowed x, to get the value
    }
    output
}
//Let's address Closure
//What if m and c are declared outside the apply function above
//and we pass only x. E.g.,

pub fn run2() {
    let c = 10;
    let m = 20;
    let xses = [1, 2, 3, 4, 5];

    //Let's use our straight_line function above. We must pass m,c and xses as arguments
    let output = straight_line_function(m, c, &xses);
    println!("Points for straight line plot are {:?}", output);

    //Let us use closure without having to pass m and c
    let strait_line_closure = |xses: &[i32]| -> Vec<(i32, i32)> {
        let mut output: Vec<(i32, i32)> = Vec::new(); //you could also use vec![] to bring in initial arguments
        for x in xses {
            let y = (m * x) + c;
            output.push((*x, y)) //here we have to dereference the borrowed x, to get the value
        }
        output
    };

    let output2 = strait_line_closure(&xses); //Can read m an y from the environment

    println!("Points for straight line plot 2 are {:?}", output2);

    //Below should report - can't capture dynamic environment in a fn item
    //m and c must be passed
    /*
    fn strait_line_function2(xses: &[i32]) -> Vec<(i32, i32)> {
        let mut output: Vec<(i32, i32)> = Vec::new(); //you could also use vec![] to bring in initial arguments
        for x in xses {
            let y = (m * x) + c;
            output.push((*x, y)) //here we have to dereference the borrowed x, to get the value
        }
        output
    };
    */
    //Notice that the above closure is reflecting the impl of trait Fn(&[i32]) -> Vec<i32, ...>
    //What are these function traits?
}

//function traits FnOnce, FnMut, Fn
//FnOnce when you can call only once.
fn consume<F>(func: F)
where
    F: FnOnce() -> String,
{
    // `func` consumes its captured variables, so it cannot be run more
    // than once.
    println!("Consumed: {}", func());

    println!("Delicious!");

    // Attempting to invoke `func()` again will throw a `use of moved
    // value` error for `func`.
}

//FnMut when you can call multiple times and mutate
fn do_twice<F>(mut func: F)
where
    F: FnMut(),
{
    func();
    func();
}

//Fn when you can call multiple times but without mutating
//"Use Fn as a bound when you want to accept a parameter of function-like type and need to call it repeatedly and without mutating state (e.g., when calling it concurrently)."
fn call_with_one<F>(func: F) -> usize
where
    F: Fn(usize) -> usize,
{
    func(1)
}

pub fn run3() {
    //illustrate FnOnce
    let x = String::from("x");
    let consume_and_return_x = move || x;
    consume(consume_and_return_x);

    // `consume_and_return_x` can no longer be invoked at this point

    //illustrate FnMut
    let mut x: usize = 1;
    {
        let add_two_to_x = || x += 2;
        do_twice(add_two_to_x);
    }

    //illustrate Fn
    let double = |x| x * 2;
    assert_eq!(call_with_one(double), 2);
}

pub fn run4() {
    /*let's see closures in action
    We will map through a collection, square each value, 
    retain only odd numbers and sum them or collect them
    */
    let my_array = [1, 2, 3, 4, 5, 6, 7];

    //data types in stack implement Copy trait by default, so I can use my_array twice below.
    let sum_of_all_even_numbers_after_square: i32 = my_array
        .into_iter()
        .map(|n| n ^ 2)
        .filter(|n| n % 2 == 0)
        .sum();

    let collection_of_all_even_numbers_after_square: Vec<i32> = my_array
        .into_iter()
        .map(|n| n ^ 2)
        .filter(|n| n % 2 == 0)
        .collect();

    println!(
        "Sum of all even numbers in array after square = {}.",
        sum_of_all_even_numbers_after_square
    );

    println!(
        "Collection of all even numbers in array after square = {:?}.",
        collection_of_all_even_numbers_after_square
    );

    //Tuples have no in-build iterator, so destructure first into vector
    let my_tuple = (1, 2, 3, 4, 5, 6, 7);
    let (a, b, c, d, e, f, g) = my_tuple;
    let my_vec = vec![a, b, c, d, e, f, g];

    //clone is used below so that I can reuse my_vec again. Vec do not implement Copy trait by default.
    let sum_of_all_even_numbers_from_tuple_after_square: i32 = my_vec
        .clone()
        .into_iter()
        .map(|n| n ^ 2)
        .filter(|n| n % 2 == 0)
        .sum();

    let collection_of_all_even_numbers_from_tuple_after_square: Vec<i32> = my_vec
        .into_iter()
        .map(|n| n ^ 2)
        .filter(|n| n % 2 == 0)
        .collect();

    println!(
        "Sum of all even numbers from tuple after square = {}.",
        sum_of_all_even_numbers_from_tuple_after_square
    );

    println!(
        "Collection of all even numbers from tuple after square = {:?}.",
        collection_of_all_even_numbers_from_tuple_after_square
    );
}
