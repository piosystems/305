#![allow(dead_code)] //This suppresses warnings when a given declared function is  not used.

use core::cmp::Ordering; //Used dor comparison of value sizes 

pub enum Comp { //Enumerate Comparison
    LessThan,
    GreaterThan,
    Equal,
}

pub enum Gender { //Enumerate Gender
    Male,
    Female,
}

#[derive(Debug)] //Decorate the struct Person. Debug is an inbuilt trait. This statement will provoke impl Debug for Person; Metaprogramming
struct Person {
    name: String,
    age: u8,
}
struct Unit;
// A unit struct
//Have no state of their own but useful for
//implementing some trait.
//E.g. struct Global is a unit struct that can implement traits like Allocator
//std::fmt::Error is a unit struct that implements
//traits like Error

//A tuple struct
struct Pair(i32, f32); //No named fields but has fields

//A struct with two fields
struct Point {
    x: f32,
    y: f32,
}

// Structs can be reused as fields of another struct. Below Point is used as datatype in Rectangle
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

pub fn run() {

    //declare a variable of type Person and assign values.
    let person = Person {
        name: String::from("Peter"),
        age: 27,
    };
    println!("{:#?}", person); //{:#?} implies pretty debug print person. :? is for debug print and :#? is for pretty debug print

    // Instantiate a unit struct
    let _unit = Unit;//As simple as that. If Unit implements some trait, then _unit will demand those implementations

    //declare a Point
    let point = Point { x: 10.3, y: 0.4 };

    //Access the fields of the point
    println!("point coordinates: ({}, {})", point.x, point.y);

    // Make a new point by using struct update syntax to use the fields of our
    // other one
    let bottom_right = Point { x: 5.2, ..point };
    // `bottom_right.y` will be the same as `point.y` because we used that field
    // from `point`
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    // Destructure the point using a let binding.
    let Point {
        x: left_edge,//left_edge here will be declared. If you use x:f32 only, x will be declared.
        y: top_edge,//top_edge here will be declared. If you use y:f32 only, y will be declared.
    } = point;
    dbg!(&left_edge,&top_edge);


    let _rectangle = Rectangle { //I used _ with rectangle to silence warning knowing that the variable is not used.
        //struct instantiation is an expression too as used below in Point
        top_left: Point {
            x: left_edge,//left_edge is available, thanks to the destructuring above
            y: top_edge,//top_edge is available, thanks to the destructuring above
        },
        bottom_right,
    };

    //Instantiate a tuple struct
    let pair = Pair(1, 0.1);
    //Access the fields of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    //Destructure a tuple struct
    let Pair(integer, decimal) = pair;
    println!("pair contains {:?} and {:?}", integer, decimal);
}

//Let's work on user-defined traits. Traits enable us achieve polymorphism.
//We are designing Shape below for the purpose of 
//specifying all expected functions and methods in any struct that implements Shape.
trait Shape {
    fn new(length: i32, width: i32, name: &'static str) -> Self;
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
struct Rect<'a> {
    length: i32,
    width: i32,
    name: &'a str,
}

impl Rect<'_> {
    //default default. Will overrides derived default if any. 
    fn default() -> Self {
        Rect {
            length: 1,
            width: 1,
            name: "default_name",
        }
    }
}

impl Shape for Rect<'_> {
    //Associated function used to create a new Shape
    fn new(length: i32, width: i32, name: &'static str) -> Self {
        Rect {
            length,
            width,
            name,
        }
    }

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
impl PartialEq for Rect<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.area() == other.area()
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl PartialOrd for Rect<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.area().partial_cmp(&other.area())
    }
    // Provided methods
    //fn lt(&self, other: &Rhs) -> bool { ... }
    //fn le(&self, other: &Rhs) -> bool { ... }
    //fn gt(&self, other: &Rhs) -> bool { ... }
    //fn ge(&self, other: &Rhs) -> bool { ... }
}


//A conversion implementation into String
//Expects a string slice with length, width, name, separated by commas
impl From<&'static str> for Rect<'_> {
    fn from<'a>(s: &'a str) -> Rect<'a> {
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

pub fn run2() {
    let rectangle1 = Rect::default();
    
    println!("{}", rectangle1.length);
    println!("{}", rectangle1.width);
    println!("{}", rectangle1.name);

    let rectangle2 = Rect::new(1, 3, "Rectangle2");
    let x = "4,5,Rectangle3";
    let rectangle3 = Rect::from(x);


















    

    //Compare using PartialOrd
    let result1 = rectangle1.partial_cmp(&rectangle2);
    println!(" result1 = {:?}", result1);

    let result2 = rectangle1.le(&rectangle2);
    println!("result2 = {:?}", result2);

    //Compare using PartialEq
    let result3 = rectangle2.eq(&rectangle3);
    println!("result3 = {:?}", result3);

    let result4 = rectangle2.ne(&rectangle3);
    println!("result4 = {:?}", result4);
}

//Exercise
/*
I need similar implementation for Circle and Triangle
Besides Area, I need Perimeter
I need comparison on the basis of Perimeter
 */
