use std::ptr;

use env::NapiEnv;
use result::{NapiError, NapiResult};
use sys;

use super::{NapiAny, NapiString, NapiValue, NapiValueInternal, NapiValueType};

#[derive(Clone, Copy, Debug)]
pub struct NapiUndefined<'env> {
    value: sys::napi_value,
    env: &'env NapiEnv,
}

impl<'env> NapiUndefined<'env> {
    pub fn new(env: &'env NapiEnv) -> NapiResult<Self> {
        let mut value = ptr::null_mut();
        env.handle_status(unsafe {
            sys::napi_get_undefined(env.as_sys_env(), &mut value)
        })?;

        Ok(Self { value, env })
    }
}

impl<'env> NapiValue<'env> for NapiUndefined<'env> {
    fn as_sys_value(&self) -> sys::napi_value {
        self.value
    }

    fn env(&self) -> &'env NapiEnv {
        self.env
    }

    fn from_sys_checked(
        env: &'env NapiEnv,
        value: sys::napi_value,
    ) -> NapiResult<Self> {
        if NapiAny::with_value(env, value).value_type()?
            != NapiValueType::Undefined
        {
            let message = NapiString::from_str(env, "Undefined expected")?;
            return Err(NapiError::type_error(env, &message));
        }

        Ok(Self { env, value })
    }
}

impl<'env> NapiValueInternal<'env> for NapiUndefined<'env> {
    fn construct(env: &'env NapiEnv, value: sys::napi_value) -> Self {
        Self { env, value }
    }
}
