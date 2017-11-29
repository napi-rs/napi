extern crate napi_sys;

mod args;
mod env;
mod result;
mod value;

pub use args::NapiArgs;
pub use env::NapiEnv;
pub use result::{NapiError, NapiErrorKind, NapiResult};
pub use value::{AsNapiObject, NapiAny, NapiArray, NapiArrayBuffer,
                NapiBoolean, NapiBuffer, NapiNull, NapiNumber, NapiObject,
                NapiString, NapiUndefined, NapiValue, NapiValueType};

pub mod sys {
    pub use napi_sys::*;
}

#[macro_export]
macro_rules! napi_callback {
    ($wrapper:ident, $handler:expr) => {
        #[no_mangle]
        pub extern "C" fn $wrapper(
            env: $crate::sys::napi_env,
            cb_info: $crate::sys::napi_callback_info,
        ) -> $crate::sys::napi_value {
            use std::error::Error;
            use std::ffi::CString;
            use std::ptr;

            use $crate::{NapiArgs, NapiEnv, NapiResult, NapiValue};
            use $crate::sys::{napi_get_undefined, napi_throw,
                              napi_throw_error, napi_value};

            let env_wrapper = NapiEnv::from(env);

            fn typecheck_result<'a, T: NapiValue<'a>>(_: &NapiResult<T>) {}

            let result = <_ as NapiArgs>::from_cb_info(&env_wrapper, cb_info)
                .and_then(|args| {
                    let result = $handler(&env_wrapper, args);
                    typecheck_result(&result);
                    result
                });

            match result {
                Ok(value) => value.as_sys_value(),
                Err(error) => {
                    if let Some(exception) = error.exception {
                        unsafe {
                            napi_throw(env, exception);
                        }
                    } else {
                        let message = format!("{}", &error);
                        let c_string =
                            CString::new(message).unwrap_or_else(|_| {
                                CString::new(error.description()).unwrap()
                            });

                        unsafe {
                            napi_throw_error(
                                env,
                                ptr::null(),
                                c_string.as_ptr(),
                            );
                        }
                    }

                    let mut result: napi_value = ptr::null_mut();
                    unsafe {
                        napi_get_undefined(env, &mut result);
                    }
                    result
                }
            }
        }
    };
}
