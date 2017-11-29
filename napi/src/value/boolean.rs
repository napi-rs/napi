use std::ptr;

use env::NapiEnv;
use result::{NapiError, NapiResult};
use sys;

use super::{NapiAny, NapiString, NapiValue, NapiValueInternal, NapiValueType};

#[derive(Clone, Copy, Debug)]
pub struct NapiBoolean<'env> {
    value: sys::napi_value,
    env: &'env NapiEnv,
}

impl<'env> NapiBoolean<'env> {
    fn new(env: &'env NapiEnv, value: bool) -> NapiResult<Self> {
        let mut sys_value = ptr::null_mut();
        env.handle_status(unsafe {
            sys::napi_get_boolean(env.as_sys_env(), value, &mut sys_value)
        })?;

        Ok(Self {
            value: sys_value,
            env,
        })
    }

    pub fn truth(env: &'env NapiEnv) -> NapiResult<Self> {
        NapiBoolean::new(env, true)
    }

    pub fn lie(env: &'env NapiEnv) -> NapiResult<Self> {
        NapiBoolean::new(env, false)
    }

    pub fn to_bool(&self) -> NapiResult<bool> {
        let mut result = false;

        self.env.handle_status(unsafe {
            sys::napi_get_value_bool(
                self.env.as_sys_env(),
                self.value,
                &mut result,
            )
        })?;

        Ok(result)
    }
}

impl<'env> NapiValue<'env> for NapiBoolean<'env> {
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
            != NapiValueType::Boolean
        {
            let message = NapiString::from_str(env, "Boolean expected")?;
            return Err(NapiError::type_error(env, &message));
        }

        Ok(Self { env, value })
    }
}

impl<'env> NapiValueInternal<'env> for NapiBoolean<'env> {
    fn construct(env: &'env NapiEnv, value: sys::napi_value) -> Self {
        Self { env, value }
    }
}
