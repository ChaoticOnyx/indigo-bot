use proc_macro::TokenStream;
use quote::quote;
use syn::{parse::Parse, parse_macro_input, DeriveInput, Ident};

struct ValidateApiSecret {
    pub varname: Ident,
}

impl Parse for ValidateApiSecret {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let varname = input.parse()?;

        Ok(Self { varname })
    }
}

#[proc_macro]
pub fn validate_api_secret(item: TokenStream) -> TokenStream {
    let ValidateApiSecret { varname } = syn::parse_macro_input!(item as ValidateApiSecret);

    let expanded = quote! {
        {
            let token = self.database.find_api_token_by_secret(#varname).await;

            let Some(token) = token else {
                return Err(ApiError::Unauthorized("invalid api secret".to_string()))
            };

            if token.is_expired() {
                return Err(ApiError::Unauthorized("invalid api secret".to_string()));
            };

            token
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(RightsFlags)]
pub fn rights_flags_derive(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let name = input.ident;

    let expanded = quote! {
        impl crate::api::models::rights::RightsFlags for #name {}
    };

    TokenStream::from(expanded)
}
