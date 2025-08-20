use syn::parse::{Parse, ParseStream};
use syn::{LitStr, Token};

pub struct ConstHashInput {
    pub string: LitStr,
    pub _as_token: Token![as],
    pub type_name: syn::Ident,
}

impl Parse for ConstHashInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let string = input.parse()?;
        let _as_token = input.parse()?;
        let type_name = input.parse()?;

        Ok(ConstHashInput {
            string,
            _as_token,
            type_name,
        })
    }
}

pub fn compute_hash(s: &str) -> u128 {
    let bytes = s.as_bytes();
    let mut hash: u128 = 0;
    for &byte in bytes {
        hash = hash.wrapping_mul(131);
        hash = hash.wrapping_add(byte as u128);
    }
    hash
}
