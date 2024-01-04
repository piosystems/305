use litrs::StringLit;
use proc_macro::TokenStream;
use quote::quote;
use syn::spanned::Spanned;

//Let's do function-like macro illustration here too
//they are similar to declarative macros but macro_rules!
//is not used for matching.
//Similar but more powerful than declarative macros
//Function-like macros are executed not at runtime
//but at compile time.
//Function-like macros take a TokenStream parameter and
//their definition manipulates that TokenStream using
//Rust code as the other two types of procedural macros do.

#[proc_macro]
pub fn sql(input: TokenStream) -> TokenStream {
    //input //Return same if nothing to be done. Obviously useless!

    //idea from https://stackoverflow.com/questions/61169932/how-do-i-get-the-value-and-type-of-a-literal-in-a-procedural-macro
    let input = input.into_iter().collect::<Vec<_>>();
    if input.len() != 1 {
        let msg = format!("expected exactly one input token, got {}", input.len());
        return quote! { compile_error!(#msg) }.into();
    }

    let string_lit = match StringLit::try_from(&input[0]) {
        // Error if the token is not a string literal
        Err(e) => return e.to_compile_error(),
        Ok(lit) => lit,
    };

    // `StringLit::value` returns the actual string value represented by the
    // literal. Quotes are removed and escape sequences replaced with the
    // corresponding value.
    let sql = string_lit.value();

    // Validate the sql statement using some logic
    // For example, check if it contains a SELECT condition
    if sql.contains("select ") {
        // Return the valid sql statement as a new TokenStream
        TokenStream::from(quote! {
            #sql
        })
    } else {
        // Return an error as a compile_error macro invocation
        syn::Error::new(sql.span(), "Invalid sql statement: missing SELECT condition")
            .to_compile_error()
            .into()
    }
}
