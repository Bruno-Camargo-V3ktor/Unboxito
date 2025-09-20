use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[proc_macro_derive(UnboxitSerialize)]
pub fn unboxit_serialize_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let struct_name = &ast.ident;
    let struct_name_str = struct_name.to_string();

    let fields = match &ast.data {
        Data::Struct(s) => match &s.fields {
            Fields::Named(fields) => &fields.named,
            _ => panic!("")
        },
        _ => panic!("")
    };

    let num_fields = fields.len();

    let serialize_fields = fields.iter().map(|field| {
        let field_name = field.ident.as_ref().unwrap();

        quote! {
            state.serialize_field(stringify!(#field_name), &self.#field_name)?;
        }
    });

    let g = quote! {
        impl unboxit::Serialize for #struct_name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: unboxit::Serializer,
            {
                use unboxit::SerializeStruct;

                let mut state = serializer.serialize_struct(#struct_name_str, #num_fields)?;

                #(#serialize_fields)*

                state.end()
            }
        }
    };

    g.into()
}
