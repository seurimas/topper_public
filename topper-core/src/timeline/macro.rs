extern crate proc_macro;
use proc_macro::TokenStream;
use syn::DeriveInput;

trait EnumFromArgs<O> {
    fn enum_from_args(observation_name: &String, arguments: Vec<String>) -> O;
}

#[proc_macro_derive(EnumFromArgs)]
pub fn derive_enum_from_args(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input);
    let output = quote! {
        impl EnumFromArgs<#ident> for #ident {
            fn enum_from_args(observation_name: &String, arguments: Vec<String>) -> #ident {
                panic!("Bad place");
            }
        }
    };
    output.into()
}
