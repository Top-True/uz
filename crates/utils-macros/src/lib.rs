use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

mod const_hash_utils;

#[proc_macro]
pub fn const_hash(input: TokenStream) -> TokenStream {
    let const_hash_utils::ConstHashInput {
        string, type_name, ..
    } = parse_macro_input!(input as const_hash_utils::ConstHashInput);
    let s = string.value();
    let hash_value = const_hash_utils::compute_hash(&s);
    let type_str = type_name.to_string();
    let output = match type_str.as_str() {
        "u8" => {
            let value = hash_value as u8;
            quote! { #value }
        }
        "u16" => {
            let value = hash_value as u16;
            quote! { #value }
        }
        "u32" => {
            let value = hash_value as u32;
            quote! { #value }
        }
        "u64" => {
            let value = hash_value as u64;
            quote! { #value }
        }
        "u128" => {
            quote! { #hash_value }
        }
        _ => {
            return syn::Error::new(
                type_name.span(),
                "Unsupported type. Only u8, u16, u32, u64, u128 are supported",
            )
            .to_compile_error()
            .into();
        }
    };
    output.into()
}
