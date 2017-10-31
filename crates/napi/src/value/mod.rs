use std::ptr;

use env::NapiEnv;
use result::NapiResult;
use sys;

mod any;
mod array;
mod boolean;
mod null;
mod number;
mod object;
mod string;
mod undefined;

pub use self::any::NapiAny;
pub use self::array::NapiArray;
pub use self::boolean::NapiBoolean;
pub use self::null::NapiNull;
pub use self::number::NapiNumber;
pub use self::object::NapiObject;
pub use self::string::NapiString;
pub use self::undefined::NapiUndefined;

pub trait NapiValue {
    fn as_sys_value(&self) -> sys::napi_value;
    fn env(&self) -> &NapiEnv;

    fn to_napi_boolean(&self) -> NapiResult<NapiBoolean> {
        coerce(self, sys::napi_coerce_to_bool, self::boolean::construct)
    }

    fn to_napi_number(&self) -> NapiResult<NapiNumber> {
        coerce(self, sys::napi_coerce_to_number, self::number::construct)
    }

    fn to_napi_object(&self) -> NapiResult<NapiObject> {
        coerce(self, sys::napi_coerce_to_object, self::object::construct)
    }

    fn to_napi_string(&self) -> NapiResult<NapiString> {
        coerce(self, sys::napi_coerce_to_string, self::string::construct)
    }

    fn as_any(&self) -> NapiAny {
        NapiAny::with_value(self.env(), self.as_sys_value())
    }
}

fn coerce<'a, T, U>(
    value: &'a T,
    napi_fn: unsafe extern "C" fn(
        sys::napi_env,
        sys::napi_value,
        *mut sys::napi_value,
    ) -> sys::napi_status,
    construct: fn(sys::napi_value, &'a NapiEnv) -> U,
) -> NapiResult<U>
where
    T: NapiValue + ?Sized,
    U: NapiValue + 'a,
{
    let env = value.env();
    let mut coerced_value = ptr::null_mut();

    env.handle_status(unsafe {
        napi_fn(env.as_sys_env(), value.as_sys_value(), &mut coerced_value)
    })?;

    Ok(construct(coerced_value, env))
}
