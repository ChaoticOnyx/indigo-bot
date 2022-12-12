use proc_macro::TokenStream;
use quote::quote;
use syn::parse::Nothing;
use syn::{
    parse::Parse, parse_macro_input, parse_quote, Attribute, ExprClosure, Fields, FieldsNamed,
    Ident, ItemStruct,
};

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

#[proc_macro]
pub fn async_closure(item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as ExprClosure);

    let inputs = item.inputs;
    let body = item.body;

    let expanded = quote! {
        {
            pub use app_shared::futures_util::FutureExt;

            |#inputs| { async move #body }.boxed()
        }
    };

    TokenStream::from(expanded)
}

/// Creates a lot of boilerplate code for loading a structure from config files.
#[proc_macro_attribute]
pub fn config(args: TokenStream, item: TokenStream) -> TokenStream {
    let mut item_struct = parse_macro_input!(item as ItemStruct);
    let _ = parse_macro_input!(args as Nothing);

    let struct_ident = item_struct.ident.clone();
    let struct_name = struct_ident.to_string();

    // Add derives
    {
        let new_attr: Attribute = parse_quote! {
            #[derive(Clone, serde::Serialize, serde::Deserialize)]
        };

        item_struct.attrs.push(new_attr);
    }

    // Add field

    let new_field: FieldsNamed = parse_quote! {
        {
            #[serde(rename(serialize = "type", deserialize = "type"))]
            __type: app_shared::config::ConfigType
        }
    };

    let Fields::Named(fields) = &mut item_struct.fields else {
        return syn::Error::new_spanned(item_struct, "structure must be named").into_compile_error().into()
    };

    fields.named.push(new_field.named.first().cloned().unwrap());

    // Add impl
    let fn_impl = quote! {
        #[app_shared::prelude::async_trait]
        impl app_shared::config::Config for #struct_ident {
            async fn get() -> Option<Self> {
                use app_shared::prelude::GlobalStateLock;

                app_shared::ConfigLoader::lock(app_macros::async_closure!(|cfg| {
                    cfg.find_config::<Self>(app_shared::config::ConfigType(String::from(#struct_name)))
                })).await
            }

            async fn save(self) -> Self {
                use app_shared::prelude::GlobalStateLock;

                app_shared::ConfigLoader::lock(app_macros::async_closure!(|cfg| {
                    cfg.save_config(self).await
                })).await
            }

            fn __type(&self) -> app_shared::config::ConfigType {
                app_shared::config::ConfigType(String::from(#struct_name))
            }
        }
    };

    let expanded = quote! {
        #item_struct
        #fn_impl
    };

    TokenStream::from(expanded)
}
