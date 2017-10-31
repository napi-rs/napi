extern crate napi_sys;

mod env;
mod result;
mod value;

pub use env::NapiEnv;
pub use result::{NapiError, NapiErrorKind, NapiResult};
pub use value::{NapiAny, NapiArray, NapiBoolean, NapiNull, NapiObject,
                NapiString, NapiUndefined, NapiValue};

pub mod sys {
    pub use napi_sys::*;
}
