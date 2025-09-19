use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

#[proc_macro_derive(UnboxitSerialize)]
pub fn unboxit_serialize_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let struct_name = &ast.ident;


    let g = quote! {
        impl unboxit::Serialize for #struct_name {
            
        }
    };
   
    g.into()
}