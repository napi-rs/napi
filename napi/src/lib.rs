extern crate napi_sys;

mod env;
mod result;
mod value;

pub use env::NapiEnv;
pub use result::{NapiError, NapiErrorKind, NapiResult};
pub use value::{AsNapiObject, NapiAny, NapiArray, NapiArrayBuffer,
                NapiBoolean, NapiBuffer, NapiNull, NapiObject, NapiString,
                NapiUndefined, NapiValue, NapiValueType};

pub mod sys {
    pub use napi_sys::*;
}
