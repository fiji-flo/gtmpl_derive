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
#[macro_use]
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;

#[proc_macro_derive(Gtmpl)]
pub fn gtmpl_derive(input: TokenStream) -> TokenStream {
    // Construct a string representation of the type definition
    let s = input.to_string();

    // Parse the string representation
    let ast = syn::parse_derive_input(&s).unwrap();

    // Build the impl
    let gen = impl_gtmpl(&ast);

    // Return the generated impl
    gen.parse().unwrap()
}

fn impl_gtmpl(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;
    // Check if derive(HelloWorld) was specified for a struct
    let to_value = match ast.body {
        syn::Body::Struct(syn::VariantData::Struct(ref body)) => {
            let fields = body.iter()
                .filter_map(|field| field.ident.as_ref())
                .map(|ident| quote! { m.insert(stringify!(#ident).to_owned(), s.#ident.into()) })
                .collect::<Vec<_>>();
            quote! { #(#fields);* }
        }
        syn::Body::Struct(syn::VariantData::Unit) => {
            quote!{}
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
