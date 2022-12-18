use proc_macro::TokenStream;
use proc_macro2::Span;
use proc_macro_crate::{crate_name, FoundCrate};
use quote::quote;
use std::collections::HashSet;
use syn::parse::{Nothing, ParseStream};
use syn::punctuated::Punctuated;
use syn::{
    parse::Parse, parse_macro_input, parse_quote, Attribute, ExprClosure, Fields, FieldsNamed,
    Ident, ItemStruct, Token,
};

fn normalize_crate(name: &str) -> Ident {
    let found_crate = crate_name(name).unwrap_or_else(|_| FoundCrate::Name(name.to_string()));

    match found_crate {
        FoundCrate::Itself => Ident::new("crate", Span::call_site()),
        FoundCrate::Name(name) => Ident::new(&name, Span::call_site()),
    }
}

struct ValidateApiSecret {
    pub varname: Ident,
}

impl Parse for ValidateApiSecret {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let varname = input.parse()?;

        Ok(Self { varname })
    }
}

#[proc_macro]
pub fn validate_api_secret(item: TokenStream) -> TokenStream {
    let ValidateApiSecret { varname } = syn::parse_macro_input!(item as ValidateApiSecret);

    let expanded = quote! {
        {
            let token = self.private_api.database.find_api_token_by_secret(#varname).await;

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
pub fn tokio_blocking(item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as ExprClosure);

    let inputs = item.inputs;
    let body = item.body;
    let shared_crate = normalize_crate("app-shared");

    let expanded = quote! {
        |#inputs| {
            #shared_crate::tokio::runtime::Handle::current().block_on(async {
                #body
            })
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
            fn get() -> Option<Self> {
                use app_shared::prelude::GlobalStateLock;

                app_shared::ConfigLoader::lock(|cfg| {
                    cfg.find_config::<Self>(app_shared::config::ConfigType(String::from(#struct_name)))
                })
            }

            fn save(self) -> Self {
                use app_shared::prelude::GlobalStateLock;

                app_shared::ConfigLoader::lock(|cfg| {
                    cfg.save_config(self)
                })
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

#[derive(Debug, Clone)]
struct GlobalArgs {
    pub impl_set: bool,
    pub impl_lock: bool,
    pub impl_clone: bool,
}

impl Parse for GlobalArgs {
    fn parse(input: ParseStream) -> syn::parse::Result<Self> {
        let vars: HashSet<String> = Punctuated::<Ident, Token![,]>::parse_terminated(input)?
            .into_iter()
            .map(|ident| ident.to_string())
            .collect();

        Ok(Self {
            impl_lock: vars.contains("lock"),
            impl_set: vars.contains("set"),
            impl_clone: vars.contains("clone"),
        })
    }
}

#[proc_macro_attribute]
pub fn global(args: TokenStream, item: TokenStream) -> TokenStream {
    let item_struct = parse_macro_input!(item as ItemStruct);
    let args = parse_macro_input!(args as GlobalArgs);

    let struct_ident = item_struct.ident.clone();
    let const_ident = Ident::new(
        &struct_ident.to_string().to_uppercase(),
        struct_ident.span(),
    );

    let shared_crate = normalize_crate("app-shared");

    let mut expanded = quote! {
        static #const_ident: #shared_crate::parking_lot::ReentrantMutex<std::cell::RefCell<Option<#struct_ident>>> = #shared_crate::parking_lot::ReentrantMutex::new(std::cell::RefCell::new(None));

        #item_struct

        impl #shared_crate::state::GlobalState for #struct_ident {
            fn get_static() -> &'static #shared_crate::parking_lot::ReentrantMutex<std::cell::RefCell<Option<Self>>> {
                &#const_ident
            }
        }
    };

    if args.impl_set {
        expanded = quote! {
            #expanded

            impl #shared_crate::state::GlobalStateSet for #struct_ident {}
        }
    }

    if args.impl_clone {
        expanded = quote! {
            #expanded

            impl #shared_crate::state::GlobalStateClone for #struct_ident {}
        }
    }

    if args.impl_lock {
        expanded = quote! {
            #expanded

            impl #shared_crate::state::GlobalStateLock for #struct_ident {}
        }
    }

    TokenStream::from(expanded)
}
