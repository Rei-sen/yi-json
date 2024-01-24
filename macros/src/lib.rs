use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

extern crate proc_macro;

#[proc_macro_derive(JsonDeserializable)]
pub fn json_deserializable_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;

    let gen = match ast.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => {
                let mut names = Vec::new();
                let field_deserialization = fields.named.iter().map(|field| {
                    let field_name = &field.ident;
                    let field_type = &field.ty;
                    names.push(field_name);
                    quote! {
                        let #field_name = value.get(stringify!(#field_name))?;
                        let #field_name = <#field_type>::from_json(#field_name)?;
                    }
                });
                quote! {
                    impl JsonDeserializable for #name {
                        fn from_json(value: &JsonValue) -> Option<Self> {
                            #(#field_deserialization)*
                            Some (Self { #( #names,)* })
                        }
                    }
                }
            }
            _ => {
                quote! { compile_error!("Deserializable trait only supports structs with named fields"); }
            }
        },
        _ => quote! {compile_error!("Deserializable trait only supports structs");},
    };

    gen.into()
}

#[proc_macro_derive(JsonSerializable)]
pub fn json_serializable_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;

    let gen = match ast.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => {
                let field_serialization = fields.named.iter().map(|field| {
                    let field_name = &field.ident;
                    quote! {
                        (stringify!(#field_name).to_string(), self.#field_name.to_json())
                    }
                });
                quote! {
                    impl JsonSerializable for #name {
                        fn to_json(&self) -> JsonValue {
                            let mut value = Vec::new();
                            #(value.push(#field_serialization);)*
                            JsonValue::Object(value)
                        }
                    }
                }
            }
            _ => {
                quote! { compile_error!("Serializable trait only supports structs with named fields"); }
            }
        },
        _ => quote! {compile_error!("Serializable trait only supports structs");},
    };

    gen.into()
}