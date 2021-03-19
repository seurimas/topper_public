extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;
use proc_macro::TokenStream;
use syn::{parse_macro_input, Data, DataEnum, DeriveInput, Fields};

#[proc_macro_derive(EnumFromArgs, attributes(skip_args))]
pub fn derive_enum_from_args(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input as DeriveInput);
    let this = ident;
    let arms: Vec<_> = match data {
        Data::Enum(DataEnum { variants, .. }) => variants
            .iter()
            .filter(|variant| variant.attrs.len() == 0)
            .map(|variant| {
                let variant_ident = &variant.ident;
                let variant_literal = proc_macro2::Literal::string(&variant.ident.to_string());
                match &variant.fields {
                    Fields::Unit => {
                        quote! { #variant_literal => #this::#variant_ident }
                    }
                    Fields::Unnamed(fields) => {
                        let args: Vec<_> = fields
                            .unnamed
                            .iter()
                            .enumerate()
                            .map(|(idx, _field)| {
                                quote! {
                                    arguments.get(#idx).unwrap().clone()
                                }
                            })
                            .collect();
                        quote! {
                            #variant_literal => #this::#variant_ident( #(#args,)* )
                        }
                    }
                    _ => panic!("Could not determine from args for {}", variant_ident),
                }
            })
            .collect(),
        _ => panic!("EnumFromArgs cannot only be derived from an enum."),
    };
    let arms_len = arms.len();
    let output = quote! {
        impl EnumFromArgs for #this {
            fn enum_from_args(observation_name: &str, arguments: Vec<String>) -> #this {
                match observation_name {
                    #(#arms,)*
                    _ => panic!("Could not determine from: {}({:?}) {}", observation_name, arguments, #arms_len),
                }
            }
        }
    };
    output.into()
}
