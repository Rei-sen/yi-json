
// macro_rules! JsonSerializable {
//     ($struct_name: ident {$(field_name:ident : $field_type:ty),*}) => {
//          impl JsonDeserializable for $struct_name {
//             fn from_json(&value: JsonValue) -> Option<Self> {
//                 $(
//                     let #field_name = value.get(#field_name)?;
//                     let #field_name = <#field_type>::from_json(#field_name)?;
//                 )*;

//                 Some($(#field_name,)*)
//             }
//          }
//     };
// }

// #[proc_macro_derive(JsonDeserializable)]
// pub fn deserializable_derive(input: TokenStream) -> TokenStream {
//     let input = syn::parse_macro_input!(input as DeriveInput);
//     let name = &input.ident;

//     let expanded = match &input.data {
//         Data::Struct(data) => match &data.fields {
//             Fields::Named(fields) => {
//                 let field_deserialization = fields.named.iter().map(|field| {
//                     let field_name = &field.ident;
//                     let field_type = &field.ty;
//                     quote! {
//                             let #field_name = value.get(stringify!(#field_name))?;
//                             let #field_name = <#field_type>::from_json(#field_name)?;
//                     }
//                 });

//                 quote! {
//                     impl JsonDeserializable for #name {
//                         fn from_json(value: &JsonValue) -> Option<Self>
//                         where Self: Sized,
//                         {
//                             if let JsonValue::Object(obj) = value {

//                                 #field_deserialization

//                                 Some(Self { #(#field_name,)*})
//                             } else {
//                                 None
//                             }
//                         }
//                     }
//                 }
//             },
//             _ => {
//                 quote! {compile_error!("Deserializable trait only supports structs with named fields");}
//             }
//         },
//         D => quote! { compile_error!("Deserializaable trait only supports structs");},
//     };

//     expanded.into()
// }
