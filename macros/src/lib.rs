extern crate proc_macro;
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;
use quote::__private::Span;
use quote::quote;
use syn::parse_macro_input;
use syn::DeriveInput;

#[proc_macro_attribute]
pub fn common(_metadata: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let output = quote! {
        #[derive(Debug, Clone, PartialEq)]
        #input
    };
    output.into()
}

#[proc_macro_attribute]
pub fn serializeable(_metadata: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let output = quote! {
        use serde::Serialize;
        #[derive(Serialize)]
        #input
    };

    output.into()
}

#[proc_macro_attribute]
pub fn deserializeable(_metadata: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let output = quote! {
        use serde::Deserialize;
        #[derive(Deserialize)]
        #input
    };
    output.into()
}

#[proc_macro_attribute]
pub fn json_parse(_metadata: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let generics = &input.generics;

    let mut lifetime = None;
    if let Some(lt) = generics.lifetimes().next() {
        let lt_ident = &lt.lifetime.ident;
        lifetime = Some(quote!(#lt_ident));
    }

    let output = if lifetime.is_some() {
        quote! {
            use serde::{Serialize, Deserialize};

            #[derive(Serialize, Deserialize)]
            #input
        }
    } else {
        let lifetime_ident = syn::Lifetime::new("'a", Span::call_site());

        let mut generics = generics.clone();
        generics.params.insert(
            0,
            syn::GenericParam::Lifetime(syn::LifetimeDef::new(lifetime_ident.clone())),
        );

        let input_with_lifetime = quote! {
            use serde::{Serialize, Deserialize};

            #[derive(Serialize, Deserialize)]
            #input
        };

        quote! {
            #input_with_lifetime
        }
    };

    output.into()
}
