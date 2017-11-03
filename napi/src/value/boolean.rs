use std::ptr;

use env::NapiEnv;
use result::NapiResult;
use sys;

use super::{NapiValue, NapiValueInternal};

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
}

impl<'env> NapiValueInternal<'env> for NapiBoolean<'env> {
    fn construct(env: &'env NapiEnv, value: sys::napi_value) -> Self {
        Self { env, value }
    }
}
