#![recursion_limit = "128"]

extern crate proc_macro;
#[macro_use]
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;
use quote::Tokens;
use syn::{Body, DeriveInput, VariantData};

#[proc_macro_derive(NapiArgs)]
pub fn napi_args(input: TokenStream) -> TokenStream {
    let ast = syn::parse_derive_input(&input.to_string()).unwrap();
    match impl_napi_args(&ast) {
        Ok(generated) => generated.parse().unwrap(),
        Err(message) => panic!(message),
    }
}

fn impl_napi_args(ast: &DeriveInput) -> Result<Tokens, &'static str> {
    let name = &ast.ident;

    let variant_data = match ast.body {
        Body::Struct(ref data) => data,
        _ => return Err("NapiArgs can only be derived for structs"),
    };

    let count = args_count(variant_data);
    let (init_list, imports) = gen_args_code(variant_data);

    let initializer = if let Some(init_list) = init_list {
        quote! { #name #init_list }
    } else {
        quote! { #name }
    };

    let (gen_lifetime, ref_lifetime) = if count > 0 {
        (quote! { <'env> }, quote! { 'env })
    } else {
        (quote!{}, quote!{})
    };

    Ok(quote! {
        impl<'env> ::napi::NapiArgs<'env> for #name #gen_lifetime {
            fn from_cb_info(
                env: & #ref_lifetime ::napi::NapiEnv,
                cb_info: ::napi::sys::napi_callback_info,
            ) -> ::napi::NapiResult<Self> {
                use ::napi::sys;
                use ::napi::{NapiError, NapiString};

                use ::std::ptr;

                #imports

                let mut argc = #count;
                let mut argv = [ptr::null_mut(); #count];

                env.handle_status(unsafe {
                    sys::napi_get_cb_info(
                        env.as_sys_env(),
                        cb_info,
                        &mut argc,
                        argv.as_mut_ptr(),
                        ptr::null_mut(),
                        ptr::null_mut(),
                    )
                })?;

                if argc != #count {
                    let message = NapiString::from_str(env, &format!(
                        "Expected {} arguments, but got {}",
                        #count,
                        argc,
                    ))?;
                    return Err(NapiError::type_error(env, &message));
                }

                Ok(#initializer)
            }
        }
    })
}

fn args_count(variant_data: &VariantData) -> usize {
    match *variant_data {
        VariantData::Struct(ref fields) | VariantData::Tuple(ref fields) => {
            fields.len()
        }
        VariantData::Unit => 0,
    }
}

fn gen_args_code(
    variant_data: &VariantData,
) -> (Option<Tokens>, Option<Tokens>) {
    match *variant_data {
        VariantData::Struct(ref fields) => {
            let inner = fields
                .iter()
                .enumerate()
                .map(|(idx, field)| {
                    let ident = field.clone().ident.unwrap();
                    quote! {
                        #ident: <_ as NapiValue>::from_sys_checked(
                            env,
                            argv[#idx],
                        )?
                    }
                })
                .collect::<Vec<_>>();

            let outer = quote! {
                { #(#inner),* }
            };

            let imports = quote! {
                use ::napi::NapiValue;
            };

            (Some(outer), Some(imports))
        }

        VariantData::Tuple(ref fields) => {
            let inner = (0..fields.len())
                .map(|idx| {
                    quote! {
                        <_ as NapiValue>::from_sys_checked(env, argv[#idx])?
                    }
                })
                .collect::<Vec<_>>();

            let outer = quote! {
                ( #(#inner),* )
            };

            (Some(outer), None)
        }

        VariantData::Unit => (None, None),
    }
}
