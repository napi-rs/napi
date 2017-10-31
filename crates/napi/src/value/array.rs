use std::ptr;

use env::NapiEnv;
use result::NapiResult;
use sys;

use super::NapiValue;

#[derive(Clone, Copy, Debug)]
pub struct NapiArray<'a> {
    value: sys::napi_value,
    env: &'a NapiEnv,
}

impl<'a> NapiArray<'a> {
    pub fn new(env: &'a NapiEnv) -> NapiResult<Self> {
        let mut value = ptr::null_mut();
        env.handle_status(unsafe {
            sys::napi_create_array(env.as_sys_env(), &mut value)
        })?;

        Ok(Self { value, env })
    }

    pub fn with_len(env: &'a NapiEnv, len: usize) -> NapiResult<Self> {
        let mut value = ptr::null_mut();
        env.handle_status(unsafe {
            sys::napi_create_array_with_length(
                env.as_sys_env(),
                len,
                &mut value,
            )
        })?;

        Ok(Self { value, env })
    }
}

impl<'a> NapiValue for NapiArray<'a> {
    fn as_sys_value(&self) -> sys::napi_value {
        self.value
    }

    fn env(&self) -> &NapiEnv {
        self.env
    }
}
