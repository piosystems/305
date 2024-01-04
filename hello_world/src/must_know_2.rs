use crate::how_you_hold_data_for_operations::derived::user_defined::{Rect, Shape};

pub fn run() {
    //Error handling
    //See my slides for references
    //panic!("Problem. You called panic");

    //Illustrate Some
    let mut v = vec!["a", "b", "c"];

    //pop an element from the vector
    let x = v.pop();

    //println!("{}", x.expect("I expected a value from my vector. You messed up!"));

    //What if we know that there is a possibility of having no Some value
    match x {
        Some(value) => println!("Popped {}", value),
        None => println!("Your vector is empty"),
    }
    //compare above to:
    let mut v2: Vec<&str> = Vec::new();

    //let mut y2 = v2.pop().unwrap(); //will panic without message because it returns an unhandled None
    //let mut y2 = v2.pop().expect("Do not call pop on an empty Vector"); //will panic and send a message

    //Exercise: How can you ensure that your program does not panic when you call a function that returns an Option?
    let y2 = match v2.pop() {
        Some(val) => val,
        None => "Empty vector",
    };
    println!("{}", y2);

    //let's use ? for Option
    let mut v3 = vec![1, 2, 3];

    let mut plus_one = || -> Option<i32> { Some(v3.pop()? + 1) };

    println!("Plus one: {}", plus_one().unwrap());
}

//Let's see Result instead of Option
//Here it returns OK value vs Err, unlike Option that returns Some value vs None

//Let's adjust the following to return Result
pub fn multiplier(numbers: &[f64]) -> f64 {
    let mut product = 1f64;
    for n in numbers {
        product = product * n;
    }
    product
}

//What if we want to return Err to the caller of this function when less
//than two arguments are passed

#[derive(Debug)]
pub struct ErrorTypes {
    pub number: u8,
    pub message: &'static str,
    pub detail: &'static str,
}

//Let's create static variables for our error types
const INVALID_ARG_LEN_ERR: ErrorTypes = ErrorTypes {
    number: 101,
    message: "Invalid Arg Length",
    detail: "Two or more arguments are expected",
};

const INVALID_ARG_TYPE_ERR: ErrorTypes = ErrorTypes {
    number: 102,
    message: "Invalid Arg Type. f64 expected",
    detail: "Invalid Arg Type. f64 expected. You must convert your arg to f64",
};

pub fn mature_multiplier(numbers: &[f64]) -> Result<f64, ErrorTypes> {
    if numbers.len() < 2 {
        return Err(INVALID_ARG_LEN_ERR);
    };
    let mut product = 1f64;
    for n in numbers {
        product *= n;
    }
    Ok(product)
}

//ownership and borrowing illustrations
//scope
pub fn run2() {
    {
        // s is not valid here, it’s not yet declared
        let s = "hello"; // s is valid from this point forward

        // do stuff with s
    } // this scope is now over, and s is no longer valid

    //above so far is all about stack. What about when we are dealing with
    //heap allocated memory where for memory saving purposes, different variable can
    //share the heap allocated memory?

    {
        let s = String::from("hello"); // s is valid from this point forward

        // do stuff with s
    } // this scope is now over, and s is no
      // longer valid

    /*
    There is a natural point at which we can return the memory our
    String needs to the allocator: when s goes out of scope.
    When a variable goes out of scope, Rust calls a special function for us.
    This function is called drop, and it’s where the author of String
    can put the code to return the memory. Rust calls drop automatically
    at the closing curly bracket.
     */

    let x = 5;
    let y = x;
    /*
    above shows: bind the value 5 to x; then make a copy of the value in x
    and bind it to y.” We now have two variables, x and y,
    and both equal 5
     */

    //Now let’s look at the String version:
    let s1 = String::from("hello");
    let s2 = s1;

    //println!("{}, world!", s1);
    //What is happening above?
}

//Declarative macros
/*
Case 1: We would like to create a macro that would allow us instantiate
one or more rectangles (along with their Shape trait impl) at a go. i.e., :
rectangles!((length:f32, width:f32, name:&str),…,n)
E.g., rectangles!((1, 1, "rect1"), (3.5, 7.0, "rect2"))
 */

#[macro_export] //in-built in Rust
macro_rules! rectangles {
    ($($rect_props_tuple:expr),*) => {
        //I want to return a Vector of Rectangles
        {
            let mut rect_vec = Vec::new();
            //take our expression received, get the length, width and name
            //from each and create the appropriate Rect and push each
            //to our rect_rec
            $(let (length, width, name) = $rect_props_tuple;
            let rect = Rect{length: length as i32,
                width: width as i32, name: name as &str};
            rect_vec.push(rect);
            )*
            rect_vec
        }

    };
}

//Try our rectangles! declarative macro.
pub fn run3(){
    let rects = rectangles!((1,1,"rect1"),(3,7,"rect2"));
    println!(
        "Area of rectangle 1 = {}; area of rectangle 2 = {}",
        rects[0].area(),
        rects[1].area()
    )
}

//You can also have multiple expressions in a declarative macro.
//What if you want a second expression that contains defaults for 
//length, width and name for the rect
//This implies that length, width and name will be optional.

#[macro_export]
macro_rules! rectangles_with_default {
    (($($rect_props_tuple:expr),*), $default_tuple:expr) => {
        {
            let mut rect_vec = Vec::new();
            let (default_length, default_width, default_name) = $default_tuple;
            $(
                let (length, width, name) = $rect_props_tuple;
                let rect = Rect{
                    length: if length.is_none(){default_length as i32} else {length.unwrap() as i32},
                    width: if width.is_none() {default_width as i32} else {width.unwrap() as i32},
                    name: if name.is_none() {default_name as &str} else {name.unwrap() as &str}
                };
                rect_vec.push(rect);
            )*
            rect_vec
        }
    };
}

pub fn run4(){
    let rects = rectangles_with_default!(
        (
            (None as Option<u32>, Some(-1), Some("rect1")),
            (Some(3.5), Some(7.0), None as Option<&str>)

        ),
        (1, 1, "default_name")
    );

    println!(
        "Area of rectangle named '{}' = {}; Area of rectangle named '{}' = {}",
        rects[0].name,
        rects[0].area(),
        rects[1].name,
        rects[1].area()
    )
}


pub fn run5() {
    println!("-----About to test Derive macro-----");
    use rect_shape::Shape;
    use rect_shape_derive::Shape;

    #[derive(Debug, Clone, Shape)]
    struct RectWithDerivedShape {
        length: f32,
        width: f32,
        name: &'static str,
    }

    //the Shape trait implementations
    //should be available for RectWithDerivedShape
    //without further explicit implementation

    let rectangle1 = RectWithDerivedShape {
        length: 1.0,
        width: 2.0,
        name: "Rect 1 with derived Shape trait",
    };
    println!(
        "Area of rectangle1 with derived Shape = {}",
        rectangle1.area()
    ); //successfully printing area will show that area was implemented eventhough we did not explicitly do so in our code.
}

pub fn run6() {
    println!("-----About to test Attribute-like macros-----");
    //next is Attribute-like macros
    use my_attribute_proc_macros::my_attribute_macro;

    #[my_attribute_macro(10)]
    fn my_ordinary_function(x: i32) -> i32 {
        x * 3
    }

    println!("{}", my_ordinary_function(3))
}

pub fn run7() {
    println!("----About to test Attribute-like macros 2----");
    //next is Attribute-like macros
    use my_attribute_proc_macros::route;

    #[route("GET", "/")]
    fn my_controller_endpoint() { 
        let header = "test header";
        header
    }

    println!("{:?}", my_controller_endpoint())
    
}

//Function-like macro
pub fn run8() {
    use my_functional_proc_macros::sql;

    println!("----About to test Function-like macros----");

    let valid_sql = sql!("select * from users");
    println!("{}", valid_sql);
}


