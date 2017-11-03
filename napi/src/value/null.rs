use std::ptr;

use env::NapiEnv;
use result::NapiResult;
use sys;

use super::{NapiValue, NapiValueInternal};

#[derive(Clone, Copy, Debug)]
pub struct NapiNull<'env> {
    value: sys::napi_value,
    env: &'env NapiEnv,
}

impl<'env> NapiNull<'env> {
    pub fn new(env: &'env NapiEnv) -> NapiResult<Self> {
        let mut value = ptr::null_mut();
        env.handle_status(
            unsafe { sys::napi_get_null(env.as_sys_env(), &mut value) },
        )?;

        Ok(Self { value, env })
    }
}

impl<'env> NapiValue<'env> for NapiNull<'env> {
    fn as_sys_value(&self) -> sys::napi_value {
        self.value
    }

    fn env(&self) -> &'env NapiEnv {
        self.env
    }
}

impl<'env> NapiValueInternal<'env> for NapiNull<'env> {
    fn construct(env: &'env NapiEnv, value: sys::napi_value) -> Self {
        Self { env, value }
    }
}
