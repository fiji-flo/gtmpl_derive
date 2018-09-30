//! This crate provides gtmpl_value's derive macro.
//!
//! ```rust
//! #[macro_use]
//! extern crate gtmpl_derive;
//! extern crate gtmpl_value;
//! use gtmpl_value::Value;
//!
//! #[derive(Gtmpl)]
//! struct Foo {
//!     bar: u8
//! }
//!
//! fn main() {
//!     let v: Value = (Foo { bar: 23 }).into();
//! }
//! ```
extern crate gtmpl_value;
extern crate proc_macro;
extern crate proc_macro2;
#[macro_use]
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;

#[proc_macro_derive(Gtmpl)]
pub fn gtmpl_derive(input: TokenStream) -> TokenStream {
    // Construct a string representation of the type definition

    // Parse the string representation
    let ast = syn::parse(input).unwrap();

    // Build the impl
    let gen = impl_gtmpl(&ast);

    // Return the generated impl
    gen.into()
}

fn impl_gtmpl(ast: &syn::DeriveInput) -> proc_macro2::TokenStream {
    let name = &ast.ident;
    // Check if derive(HelloWorld) was specified for a struct
    let to_value = match ast.data {
        syn::Data::Struct(syn::DataStruct { ref fields, .. }) => {
            let fields = fields
                .iter()
                .filter_map(|field| field.ident.as_ref())
                .map(|ident| quote! { m.insert(stringify!(#ident).to_owned(), s.#ident.into()) })
                .collect::<Vec<_>>();
            quote! { #(#fields);* }
        }
        _ => {
            //Nope. This is an Enum. We cannot handle these!
            panic!("#[derive(Gtmpl)] is only defined for structs, not for enums!");
        }
    };
    quote! {
        impl From<#name> for ::gtmpl_value::Value {
            fn from(s: #name) -> ::gtmpl_value::Value {
                use std::collections::HashMap;
                #[warn(unused_mut)]
                let mut m: HashMap<String, ::gtmpl_value::Value> = HashMap::new();
                #to_value;
                ::gtmpl_value::Value::Object(m)
            }
        }
    }
}
