use std::ptr;

use env::NapiEnv;
use result::NapiResult;
use sys;

use super::NapiValue;

#[derive(Clone, Copy, Debug)]
pub struct NapiBoolean {
    value: sys::napi_value,
}

impl NapiBoolean {
    fn new(env: &NapiEnv, value: bool) -> NapiResult<Self> {
        let mut sys_value = ptr::null_mut();
        env.handle_status(unsafe {
            sys::napi_get_boolean(env.as_sys_env(), value, &mut sys_value)
        })?;
        Ok(Self { value: sys_value })
    }

    pub fn truth(env: &NapiEnv) -> NapiResult<Self> {
        NapiBoolean::new(env, true)
    }

    pub fn lie(env: &NapiEnv) -> NapiResult<Self> {
        NapiBoolean::new(env, false)
    }
}

impl NapiValue for NapiBoolean {
    fn as_sys_value(&self) -> sys::napi_value {
        self.value
    }

    fn from_sys_value(value: sys::napi_value) -> Self {
        Self { value }
    }
}
