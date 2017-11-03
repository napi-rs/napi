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

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum NapiValueType {
    Undefined,
    Null,
    Boolean,
    Number,
    String,
    Symbol,
    Object,
    Function,
    External,
}

pub trait NapiValue<'env> {
    fn as_sys_value(&self) -> sys::napi_value;
    fn env(&self) -> &'env NapiEnv;

    fn to_napi_boolean(&self) -> NapiResult<NapiBoolean<'env>> {
        coerce(self, sys::napi_coerce_to_bool)
    }

    fn to_napi_number(&self) -> NapiResult<NapiNumber<'env>> {
        coerce(self, sys::napi_coerce_to_number)
    }

    fn to_napi_object(&self) -> NapiResult<NapiObject<'env>> {
        coerce(self, sys::napi_coerce_to_object)
    }

    fn to_napi_string(&self) -> NapiResult<NapiString<'env>> {
        coerce(self, sys::napi_coerce_to_string)
    }

    fn as_napi_any(&self) -> NapiAny<'env> {
        NapiAny::with_value(self.env(), self.as_sys_value())
    }

    fn value_type(&self) -> NapiResult<NapiValueType> {
        let env = self.env();
        let mut result = sys::napi_valuetype::napi_undefined;

        env.handle_status(unsafe {
            sys::napi_typeof(env.as_sys_env(), self.as_sys_value(), &mut result)
        })?;

        Ok(match result {
            sys::napi_valuetype::napi_undefined => NapiValueType::Undefined,
            sys::napi_valuetype::napi_null => NapiValueType::Null,
            sys::napi_valuetype::napi_boolean => NapiValueType::Boolean,
            sys::napi_valuetype::napi_number => NapiValueType::Number,
            sys::napi_valuetype::napi_string => NapiValueType::String,
            sys::napi_valuetype::napi_symbol => NapiValueType::Symbol,
            sys::napi_valuetype::napi_object => NapiValueType::Object,
            sys::napi_valuetype::napi_function => NapiValueType::Function,
            sys::napi_valuetype::napi_external => NapiValueType::External,
        })
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

    fn strict_equals<T>(&self, other: &T) -> NapiResult<bool>
    where
        T: NapiValue<'env> + ?Sized,
    {
        let env = self.env();
        let mut result = false;

        env.handle_status(unsafe {
            sys::napi_strict_equals(
                env.as_sys_env(),
                self.as_sys_value(),
                other.as_sys_value(),
                &mut result,
            )
        })?;

        Ok(result)
    }
}

pub trait AsNapiObject<'env>: NapiValue<'env> {
    fn as_napi_object(&self) -> NapiObject<'env> {
        NapiObject::construct(self.env(), self.as_sys_value())
    }
}

trait NapiValueInternal<'env>: NapiValue<'env> {
    fn construct(env: &'env NapiEnv, value: sys::napi_value) -> Self;
}

fn coerce<'env, T, U>(
    value: &T,
    napi_fn: unsafe extern "C" fn(
        sys::napi_env,
        sys::napi_value,
        *mut sys::napi_value,
    ) -> sys::napi_status,
) -> NapiResult<U>
where
    T: NapiValue<'env> + ?Sized,
    U: NapiValueInternal<'env>,
{
    let env = value.env();
    let mut coerced_value = ptr::null_mut();

    env.handle_status(unsafe {
        napi_fn(env.as_sys_env(), value.as_sys_value(), &mut coerced_value)
    })?;

    Ok(U::construct(env, coerced_value))
}

fn check_type<'env, T>(
    value: &T,
    napi_fn: unsafe extern "C" fn(sys::napi_env, sys::napi_value, *mut bool)
        -> sys::napi_status,
) -> NapiResult<bool>
where
    T: NapiValue<'env> + ?Sized,
{
    let env = value.env();
    let mut result = false;

    env.handle_status(unsafe {
        napi_fn(env.as_sys_env(), value.as_sys_value(), &mut result)
    })?;

    Ok(result)
}
