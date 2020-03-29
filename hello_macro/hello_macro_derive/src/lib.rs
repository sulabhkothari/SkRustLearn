extern crate proc_macro;

use crate::proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_hello_macro(&ast)
}

// DeriveInput {
//    // --snip--
//
//    ident: Ident {
//        ident: "Pancakes",
//        span: #0 bytes(95..103)
//    },
//    data: Struct(
//        DataStruct {
//            struct_token: Struct,
//            fields: Unit,
//            semi_token: Some(
//                Semi
//            )
//        }
//    )
//}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}", stringify!(#name));
            }
        }
    };
    gen.into()
}

// Macro-Rules:
// #[macro_export]
// macro_rules! vec {
//    ( $( $x:expr ),* ) => {
//        {
//            let mut temp_vec = Vec::new();
//            $(
//                temp_vec.push($x);
//            )*
//            temp_vec
//        }
//    };
// }

// Attribute-like macros are similar to custom derive macros, but instead of generating code for the
// derive attribute, they allow you to create new attributes. They’re also more flexible: derive only
// works for structs and enums; attributes can be applied to other items as well, such as functions.
// #[route(GET, "/")]
// fn index() {
//
// #[proc_macro_attribute]
// pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {
// Other than that, attribute-like macros work the same way as custom derive macros: you create a
// crate with the proc-macro crate type and implement a function that generates the code you want!

// Function-like macros:
// Function-like macros define macros that look like function calls. Similarly to macro_rules! macros,
// they’re more flexible than functions; for example, they can take an unknown number of arguments.
// However, macro_rules! macros can be defined only using the match-like syntax.
//  An example of a function-like macro is an sql! macro that might be called like so:
// let sql = sql!(SELECT * FROM posts WHERE id=1);
// This macro would parse the SQL statement inside it and check that it’s syntactically correct,
// which is much more complex processing than a macro_rules! macro can do. The sql! macro would be
// defined like this:
// #[proc_macro]
// pub fn sql(input: TokenStream) -> TokenStream {