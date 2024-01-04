use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn, AttributeArgs};

#[proc_macro_attribute]
pub fn my_attribute_macro(attr: TokenStream, item: TokenStream) -> TokenStream {
    //Get the original function
    let input = parse_macro_input!(item as ItemFn);
    //Get the original function name
    let name = input.sig.ident;
    //Get the arguments to the original function
    let args = input.sig.inputs.clone();
    //Get the function block {}
    let block = input.block;
    //Get the attributes
    let attr_args = parse_macro_input!(attr as AttributeArgs);

    /*
    Below is illustration for taking an attribute and adding it to outcome of function
    as exemplified by the commented out use here
    #[my_attribute_macro(6)]
    fn my_ordinary_function(x: i32) -> i32 {
        x * 3
    }
    fn main() {
        println!("{}", my_ordinary_function(3));
    }
     */
    let n = attr_args[0].clone(); //assuming that you have an attribute number, you want to add to original statement.
    let output = quote! {
        fn #name(#args) -> i32 {
            (#block) + #n
        }
    };
    output.into()
}

#[proc_macro_attribute]
pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {
    //Get the original function
    let input = parse_macro_input!(item as ItemFn);
    //Get the original function name
    let name = input.sig.ident;
    //Get the arguments to the original function, if needed
    //let args = input.sig.inputs.clone();
    //Get the function block {}
    let block = input.block;
    //Get the attributes
    let attr_args = parse_macro_input!(attr as AttributeArgs);

    /*
    Below is illustration for taking an attribute capturing
    parameters e.g. in route definition when creating a Web framework
    and using them in the logic. i.e.
    #[route(GET, "/")]
     */
    let http_verb = attr_args[0].clone(); //assuming that you have an attribute like "GET"
    let url_route = attr_args[1].clone();

    
    let output = quote! {
        fn #name<'a>() -> (&'a str, &'a str, &'a str) {
            //https://github.com/smoqadam/rust-router
            let x = 1; // <- your extra code, if you want
            //do what you want with the variables passed
            let http_verb = #http_verb;
            let url_route = #url_route;
            //println!("Testing http_verb: {:}", http_verb);
            //println!("Testing url_route: {:}", url_route);

            //#(#stmts)* //call the original statements and return the original outcome
            (#block, #http_verb, #url_route) //return the original outcome
        }
    };
    output.into()
}
