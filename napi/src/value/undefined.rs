use std::ptr;

use env::NapiEnv;
use result::NapiResult;
use sys;

use super::{NapiValue, NapiValueInternal};

#[derive(Clone, Copy, Debug)]
pub struct NapiUndefined<'a> {
    value: sys::napi_value,
    env: &'a NapiEnv,
}

impl<'a> NapiUndefined<'a> {
    pub fn new(env: &'a NapiEnv) -> NapiResult<Self> {
        let mut value = ptr::null_mut();
        env.handle_status(unsafe {
            sys::napi_get_undefined(env.as_sys_env(), &mut value)
        })?;

        Ok(Self { value, env })
    }
}

impl<'a> NapiValue<'a> for NapiUndefined<'a> {
    fn as_sys_value(&self) -> sys::napi_value {
        self.value
    }

    fn env(&self) -> &'a NapiEnv {
        self.env
    }
}

impl<'a> NapiValueInternal<'a> for NapiUndefined<'a> {
    fn construct(env: &'a NapiEnv, value: sys::napi_value) -> Self {
        Self { env, value }
    }
}
