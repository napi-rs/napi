use std::ptr;

use env::NapiEnv;
use result::NapiResult;
use sys;

mod array;
mod boolean;
mod number;
mod object;
mod string;
mod undefined;

pub use self::array::NapiArray;
pub use self::boolean::NapiBoolean;
pub use self::number::NapiNumber;
pub use self::object::NapiObject;
pub use self::string::NapiString;
pub use self::undefined::NapiUndefined;

macro_rules! coercion_method {
    ($name:ident, $type:ident, $napi_fn:expr) => {
        fn $name(&self, env: &NapiEnv) -> NapiResult<$type> {
            let mut value = ptr::null_mut();
            let status = unsafe {
                $napi_fn(
                    env.as_sys_env(),
                    self.as_sys_value(),
                    &mut value,
                )
            };
            env.handle_status(status)?;
            Ok($type::from_sys_value(value))
        }
    }
}

pub trait NapiValue {
    fn as_sys_value(&self) -> sys::napi_value;
    fn from_sys_value(value: sys::napi_value) -> Self;

    coercion_method!(to_napi_boolean, NapiBoolean, sys::napi_coerce_to_bool);
    coercion_method!(to_napi_number, NapiNumber, sys::napi_coerce_to_number);
    coercion_method!(to_napi_object, NapiObject, sys::napi_coerce_to_object);
    coercion_method!(to_napi_string, NapiString, sys::napi_coerce_to_string);
}
