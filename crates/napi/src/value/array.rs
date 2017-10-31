use std::ptr;

use env::NapiEnv;
use result::NapiResult;
use sys;

use super::NapiValue;

#[derive(Clone, Copy, Debug)]
pub struct NapiArray {
    value: sys::napi_value,
}

impl NapiArray {
    pub fn new(env: &NapiEnv) -> NapiResult<Self> {
        let mut value = ptr::null_mut();
        env.handle_status(unsafe {
            sys::napi_create_array(env.as_sys_env(), &mut value)
        })?;

        Ok(Self { value })
    }

    pub fn with_len(env: &NapiEnv, len: usize) -> NapiResult<Self> {
        let mut value = ptr::null_mut();
        env.handle_status(unsafe {
            sys::napi_create_array_with_length(
                env.as_sys_env(),
                len,
                &mut value,
            )
        })?;

        Ok(Self { value })
    }
}

impl NapiValue for NapiArray {
    fn as_sys_value(&self) -> sys::napi_value {
        self.value
    }

    fn from_sys_value(value: sys::napi_value) -> Self {
        Self { value }
    }
}
