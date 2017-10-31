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

fn coerce<T, U>(
    value: &T,
    env: &NapiEnv,
    napi_fn: unsafe extern "C" fn(
        sys::napi_env,
        sys::napi_value,
        *mut sys::napi_value,
    ) -> sys::napi_status,
) -> NapiResult<U>
where
    T: NapiValue + ?Sized,
    U: NapiValue,
{
    let mut coerced_value = ptr::null_mut();

    env.handle_status(unsafe {
        napi_fn(env.as_sys_env(), value.as_sys_value(), &mut coerced_value)
    })?;

    Ok(U::from_sys_value(coerced_value))
}

pub trait NapiValue {
    fn as_sys_value(&self) -> sys::napi_value;
    fn from_sys_value(value: sys::napi_value) -> Self;

    fn to_napi_boolean(&self, env: &NapiEnv) -> NapiResult<NapiBoolean> {
        coerce(self, env, sys::napi_coerce_to_bool)
    }

    fn to_napi_number(&self, env: &NapiEnv) -> NapiResult<NapiNumber> {
        coerce(self, env, sys::napi_coerce_to_number)
    }

    fn to_napi_object(&self, env: &NapiEnv) -> NapiResult<NapiObject> {
        coerce(self, env, sys::napi_coerce_to_object)
    }

    fn to_napi_string(&self, env: &NapiEnv) -> NapiResult<NapiString> {
        coerce(self, env, sys::napi_coerce_to_string)
    }
}
