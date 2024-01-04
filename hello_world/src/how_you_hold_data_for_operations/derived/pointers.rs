use core::cmp::Ordering;

pub fn run() {
    let mut x = 10;

    println!("x before change = {}", x);

    let y = &mut x; //y is a mutable reference to x
    let z: *const u32 = y; //z is an immutable raw pointer to y which references x
    //let a = y as *mut u32; //a is a mutable raw pointer to y which references x
    let a: *mut u32 = y; //a is a mutable raw pointer to y which references x. Same as previous line

    println!("y = {:?}", y); //expect value in x
    println!("z = {:?}", z); //expect memory address
    println!("a = {:?}", a); //expect same memory address as z above

    *y = 11; //expect value in x to change
    println!("x after first change = {}", x);

    unsafe {
        *a = 12; //expect value in x to change
        assert!(x == 12)
    };

    println!("x after second change = {}", x);
}

//Smart pointers
//1. Box
//"All values in Rust are stack allocated by default. 
//Values can be boxed (allocated on the heap) by creating a Box<T>. 
//A box is a smart pointer to a heap allocated value of type T. 
//When a box goes out of scope, its destructor is called, 
//the inner object is destroyed, 
//and the memory on the heap is freed."
//See https://doc.rust-lang.org/rust-by-example/std/box.html

//Like any other pointer, Box value can be dereferenced using the * operator

//Lets force our Rect to be on the heap
pub trait Shape {
    //Moved below from trait to object directly so as not to conflict with Box::new
    //fn new(length: i32, width: i32, name: &'static str) -> Self;
    fn area(&self) -> i32;
    fn set_length(&mut self, length: i32);
    fn get_length(&self) -> i32;
    fn set_width(&mut self, width: i32);
    fn get_width(&self) -> i32;
    fn set_name(&mut self, name: &'static str);
    fn get_name(&self) -> &str;
}
//The use of 'static lifetime above ensures that our
//compiler is clear about the availability of those values, as they are borrowed.
//static will be available throughout the lifetime of the program.

///Use Default to specify the availability of default instance creation without values passed for property
#[derive(Default, Debug, Clone)]
pub struct Rect {
    pub length: i32,
    pub width: i32,
    pub name: &'static str,
}

impl Rect {
    //default default() function. Will override derived default if any. 
    fn default() -> Self {
        Rect {
            length: 1,
            width: 1,
            name: "default_name",
        }
    }

    fn new(length: i32, width: i32, name: &'static str) -> Self {
        Rect {
            length,
            width,
            name,
        }
    }
}

impl Shape for Rect {
    //Associated function used to create a new Shape
    //Moved below from trait to object directly so as not to conflict with Box::new
    /* 
    fn new(length: i32, width: i32, name: &'static str) -> Self {
        Rect {
            length,
            width,
            name,
        }
    }*/

    fn area(&self) -> i32 {
        self.length * self.width
    }

    fn set_length(&mut self, length: i32) {
        self.length = length;
    }

    fn get_length(&self) -> i32 {
        self.length
    }

    fn set_width(&mut self, width: i32) {
        self.width = width;
    }

    fn get_width(&self) -> i32 {
        self.width
    }

    fn set_name(&mut self, name: &'static str) {
        self.name = name;
    }

    fn get_name(&self) -> &str {
        self.name
    }
}

//implement Partial Eq
impl PartialEq for Rect {
    fn eq(&self, other: &Self) -> bool {
        self.area() == other.area()
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl PartialOrd for Rect {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.area().partial_cmp(&other.area())
    }
    // Provided methods
    //fn lt(&self, other: &Rhs) -> bool { ... }
    //fn le(&self, other: &Rhs) -> bool { ... }
    //fn gt(&self, other: &Rhs) -> bool { ... }
    //fn ge(&self, other: &Rhs) -> bool { ... }
}


//Create a Rect from a string slice
//Expects a string slice with length, width, name, separated by commas
impl From<&'static str> for Rect {
    fn from(s: &'static str) -> Rect {
        let mut parts = s.split(',');
        let length = match parts.next() {
            Some(val) => val.parse::<i32>().unwrap(),
            None => 0,
        };
        let width = match parts.next() {
            Some(val) => val.parse::<i32>().unwrap(),
            None => 0,
        };
        let name = match parts.next() {
            Some(val) => val,
            None => "",
        };
        Rect { length, width, name: &name }
    }
}

//A conversion implementation into String. Can be used to move the variable into another
impl Into<String> for Rect {
    fn into(self) -> String {
        //Let's return a string template literal
        format!("My name is {} and my area is {}.", self.name, self.area())
    }
}

struct Circle {
    radius: f32
}

//A conversion implementation into Circle for Rect. Can be used to move the variable into another
impl Into<Circle> for Rect {
    fn into(self) -> Circle {
        //Let's return a string template literal
        let radius = f32::sqrt(self.area() as f32 / 3.141592653589793238462643383279502884197);
        Circle {radius}
    }
}

pub fn run2(){
    //Static vs Dynamic dispatch. See https://reintech.io/blog/understanding-implementing-rust-static-dynamic-dispatch
    //static dispatch. Size of Rect is known at compile time.
    let mut rectangle1: Box<Rect> = Box::new(Rect {
        length: 12,
        width: 9,
        name: "Rectangle 1",
    });

    //Construct a Trait Object. if your function returns a pointer-to-trait-on-heap in this way, you need to write the return type with the dyn keyword, e.g. Box<dyn Animal>. See https://doc.rust-lang.org/rust-by-example/trait/dyn.html
    //Shape must be object safe, hence no associated function allowed. All must have self as arguments
    //Dynamic typing in Rust. dyn is a prefix of a trait object’s type.
    //More realistically polymorphic as the size of Shape will be dynamically determined.
    //Read about Dynamic dispatch https://en.wikipedia.org/wiki/Dynamic_dispatch
    let mut rectangle2: Box<dyn Shape> = Box::new(Rect {
        length: 5,
        width: 9,
        name: "Rectangle 2",
    });

    //Below is also dynamic dispatch but does not require heap
    let mut rectangle3: &dyn Shape = &Rect {
        length: 5,
        width: 9,
        name: "Rectangle 2",
    };

    println!("Area of {} = {}", rectangle1.name, rectangle1.area());//rectangle1.area() is statically dispatched
    println!("Area of {} = {}", rectangle2.get_name(), rectangle2.area());//rectangle2.area() is dynamically dispatched. Direct use of name is not available.
    println!("Area of {} = {}", rectangle3.get_name(), rectangle2.area());//rectangle2.area() is dynamically dispatched. Direct use of name is not available.
    //Static Dispatch is more efficient, but it can lead to code bloat because the compiler generates a unique function for each data type. Dynamic Dispatch is less efficient but can be more flexible and can reduce code bloat.
    //See https://reintech.io/blog/understanding-implementing-rust-static-dynamic-dispatch

    //Smart Pointers continue.
    //Move Box
    {
        rectangle1;
    }
    //No longer possible to use rectangle1 below. Single owner
    //println!("Area of {} = {}", rectangle1.name, rectangle1.area());//rectangle1.area() is statically dispatched

    
}

