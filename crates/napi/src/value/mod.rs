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
        coerce(self, sys::napi_coerce_to_bool)
    }

    fn to_napi_number(&self) -> NapiResult<NapiNumber> {
        coerce(self, sys::napi_coerce_to_number)
    }

    fn to_napi_object(&self) -> NapiResult<NapiObject> {
        coerce(self, sys::napi_coerce_to_object)
    }

    fn to_napi_string(&self) -> NapiResult<NapiString> {
        coerce(self, sys::napi_coerce_to_string)
    }

    fn as_any(&self) -> NapiAny {
        NapiAny::with_value(self.env(), self.as_sys_value())
    }

    fn instanceof(&self, constructor: &NapiObject) -> NapiResult<bool> {
        let env = self.env();
        let mut result = false;

        env.handle_status(unsafe {
            sys::napi_instanceof(
                env.as_sys_env(),
                self.as_sys_value(),
                constructor.as_sys_value(),
                &mut result,
            )
        })?;

        Ok(result)
    }

    fn is_array(&self) -> NapiResult<bool> {
        check_type(self, sys::napi_is_array)
    }

    fn is_arraybuffer(&self) -> NapiResult<bool> {
        check_type(self, sys::napi_is_arraybuffer)
    }

    fn is_buffer(&self) -> NapiResult<bool> {
        check_type(self, sys::napi_is_buffer)
    }

    fn is_error(&self) -> NapiResult<bool> {
        check_type(self, sys::napi_is_error)
    }

    fn is_typedarray(&self) -> NapiResult<bool> {
        check_type(self, sys::napi_is_typedarray)
    }

    fn is_dataview(&self) -> NapiResult<bool> {
        check_type(self, sys::napi_is_dataview)
    }
}

trait NapiValueInternal<'a>: NapiValue + 'a {
    fn construct(env: &'a NapiEnv, value: sys::napi_value) -> Self;
}

fn coerce<'a, T, U>(
    value: &'a T,
    napi_fn: unsafe extern "C" fn(
        sys::napi_env,
        sys::napi_value,
        *mut sys::napi_value,
    ) -> sys::napi_status,
) -> NapiResult<U>
where
    T: NapiValue + ?Sized,
    U: NapiValueInternal<'a>,
{
    let env = value.env();
    let mut coerced_value = ptr::null_mut();

    env.handle_status(unsafe {
        napi_fn(env.as_sys_env(), value.as_sys_value(), &mut coerced_value)
    })?;

    Ok(U::construct(env, coerced_value))
}

fn check_type<'a, T>(
    value: &'a T,
    napi_fn: unsafe extern "C" fn(sys::napi_env, sys::napi_value, *mut bool)
        -> sys::napi_status,
) -> NapiResult<bool>
where
    T: NapiValue + ?Sized,
{
    let env = value.env();
    let mut result = false;

    env.handle_status(unsafe {
        napi_fn(env.as_sys_env(), value.as_sys_value(), &mut result)
    })?;

    Ok(result)
}
