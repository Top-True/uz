use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{FieldsNamed, Token, Visibility, parse_macro_input};

struct SpeciesDefine {
    visibility: Visibility,
    name: Ident,
}

impl Parse for SpeciesDefine {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let visibility = Visibility::parse(input)?;
        let name = Ident::parse(input)?;
        let _lt = input.parse::<Token![<]>()?;
        while !input.peek(Token![>]) {
            if input.peek(Token![>]) {
                break;
            }
        }
        let _gt = input.parse::<Token![>]>()?;
        FieldsNamed::parse(input)?;
        Ok(Self { visibility, name })
    }
}

#[proc_macro]
pub fn species(input: TokenStream) -> TokenStream {
    let species_define = parse_macro_input!(input as SpeciesDefine);
    let species_define_visibility = species_define.visibility;
    let species_define_name = species_define.name;
    quote! {
        #[allow(non_upper_case_globals)]
        #species_define_visibility const #species_define_name: uz::core::species::MetaInfo = uz::core::species::MetaInfo {
            id: uz::core::species::ID(uz::utils::const_random!(u128)),
            children: Vec::new(),
            fields: Vec::new(),
        };
    }
    .into()
}
