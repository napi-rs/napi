use std::ptr;

use env::NapiEnv;
use result::NapiResult;
use sys;

use super::NapiValue;

#[derive(Clone, Copy, Debug)]
pub struct NapiUndefined {
    value: sys::napi_value,
}

impl NapiUndefined {
    pub fn new(env: &NapiEnv) -> NapiResult<Self> {
        let mut value = ptr::null_mut();
        env.handle_status(unsafe {
            sys::napi_get_undefined(env.as_sys_env(), &mut value)
        })?;

        Ok(Self { value })
    }
}

impl NapiValue for NapiUndefined {
    fn as_sys_value(&self) -> sys::napi_value {
        self.value
    }

    fn from_sys_value(value: sys::napi_value) -> Self {
        Self { value }
    }
}
