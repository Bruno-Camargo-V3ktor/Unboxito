use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, Lit, Meta, parse_macro_input};

fn get_field_rename(field: &syn::Field) -> Option<String> {
    for attr in &field.attrs {
        if !attr.path().is_ident("unboxit") {
            continue;
        }

        if let Meta::List(list) = &attr.meta {
            let mut renamed_value = None;
            let _ = list.parse_nested_meta(|meta| {
                if meta.path.is_ident("rename") {
                    if let Ok(expr) = meta.value()?.parse::<syn::Expr>() {
                        if let syn::Expr::Lit(syn::ExprLit {
                            lit: Lit::Str(lit), ..
                        }) = expr
                        {
                            renamed_value = Some(lit.value());
                        }
                    }
                }
                Ok(())
            });

            if renamed_value.is_some() {
                return renamed_value;
            }
        }
    }

    None
}

#[proc_macro_derive(UnboxitSerialize, attributes(unboxit))]
pub fn unboxit_serialize_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let struct_name = &ast.ident;
    

    let body = match &ast.data {
        Data::Struct(s) => match &s.fields {
            Fields::Named(fields) => {
                let num_fields = fields.named.len();
                let struct_name_str = struct_name.to_string();

                let serialize_fields = fields.named.iter().map(|field| {
                    let field_name = field.ident.as_ref().unwrap();

                    let field_name_str = match get_field_rename(field) {
                        Some(renamed) => quote! { #renamed },
                        None => quote! { stringify!(#field_name) },
                    };

                    quote! {
                        state.serialize_field(#field_name_str, &self.#field_name)?;
                    }
                });

                quote! {
                    let mut state = serializer.serialize_struct(#struct_name_str, #num_fields)?;
                    #(#serialize_fields)*
                    state.end()
                }
            }

            Fields::Unnamed(fields) => {
                let num_fields = fields.unnamed.len();
                if num_fields == 1 {
                    quote! { self.0.serialize(serializer) }
                } else {
                    let serialize_elements = (0..num_fields).map(|i| {
                        let index = syn::Index::from(i);
                        quote! { seq.serialize_element(&self.#index)?; }
                    });

                    quote! {
                        let mut seq = serializer.serialize_seq(Some(#num_fields))?;
                        #(#serialize_elements)*
                        seq.end()
                    }
                }
            }

            Fields::Unit => {
                let struct_name_str = struct_name.to_string();
                quote! { serializer.serialize_unit_struct(#struct_name_str) }
            }

        },

        _ => panic!(""),
    };

    let g = quote! {
        impl unboxit::Serialize for #struct_name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: unboxit::Serializer,
            {
                use unboxit::SerializeStruct;
                use unboxit::SerializeSeq;

                #body
            }
        }
    };

    g.into()
}
