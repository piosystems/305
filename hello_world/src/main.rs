mod greetings;
mod how_you_hold_data_for_operations;
pub mod must_know_2;

//use greetings::default_greeting;
//use greetings::default_greeting2;
//use greetings::*;

extern crate hello_world_lib;

use greetings::{french, spanish, english};

fn main() {

    let mut greeting = "Hello there";

    println!("{}", greeting);
    
    greeting = "Hello there again";

    println!("{}", greeting);

    println!("Hello, world!");
    println!("{}", english::default_greeting());
    println!("{}", english::default_greeting2());
    println!(
        "My first greeting is '{}' and the second is '{}'",
        english::default_greeting(),
        english::default_greeting2()
    );
    println!("{}", spanish::default_greeting());
    println!("{}", french::default_greeting());
    println!("{}", hello_world_lib::greeting_from_lib());
    how_you_hold_data_for_operations::primitives::scalar::run();
    how_you_hold_data_for_operations::derived::user_defined::run();
    how_you_hold_data_for_operations::derived::user_defined::run2();
    how_you_hold_data_for_operations::derived::pointers::run2();
    how_you_hold_data_for_operations::derived::error_handling::run();

    //Let's test our derived macro use
    must_know_2::run5();

    //Let's test our my_attribute_like_macros
    must_know_2::run6();
    must_know_2::run7();
    must_know_2::run8();


}
