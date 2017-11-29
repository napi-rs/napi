#![recursion_limit = "128"]

extern crate proc_macro;
#[macro_use]
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;

#[proc_macro_derive(NapiArgs)]
pub fn napi_args(input: TokenStream) -> TokenStream {
    let ast = syn::parse_derive_input(&input.to_string()).unwrap();
    match impl_napi_args(&ast) {
        Ok(generated) => generated.parse().unwrap(),
        Err(message) => panic!(message),
    }
}

fn impl_napi_args(
    ast: &syn::DeriveInput,
) -> Result<quote::Tokens, &'static str> {
    let name = &ast.ident;

    let variant_data = match ast.body {
        syn::Body::Struct(ref data) => data,
        _ => return Err("NapiArgs can only be derived for structs"),
    };

    let fields = match variant_data {
        &syn::VariantData::Tuple(ref fields) => fields,
        _ => return Err("NapiArgs can only be derived for tuple-structs"),
    };
    let count = fields.len();

    Ok(quote! {
        impl NapiArgs for #name {
            fn from_cb_info(
                env: &::napi::NapiEnv,
                cb_info: ::napi::sys::napi_callback_info,
            ) -> ::napi::NapiResult<Self> {
                use ::napi::sys;
                use ::napi::{NapiError, NapiString};

                use ::std::ptr;

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

                Ok(#name())
            }
        }
    })
}
