use std::ptr;

use env::NapiEnv;
use result::NapiResult;
use sys;

use super::NapiValue;

#[derive(Clone, Copy, Debug)]
pub struct NapiNumber {
    value: sys::napi_value,
}

impl NapiNumber {
    pub fn from_i32(env: &NapiEnv, value: i32) -> NapiResult<Self> {
        let mut sys_value = ptr::null_mut();
        env.handle_status(unsafe {
            sys::napi_create_int32(env.as_sys_env(), value, &mut sys_value)
        })?;

        Ok(Self { value: sys_value })
    }

    pub fn from_i64(env: &NapiEnv, value: i64) -> NapiResult<Self> {
        let mut sys_value = ptr::null_mut();
        env.handle_status(unsafe {
            sys::napi_create_int64(env.as_sys_env(), value, &mut sys_value)
        })?;

        Ok(Self { value: sys_value })
    }

    pub fn from_f64(env: &NapiEnv, value: f64) -> NapiResult<Self> {
        let mut sys_value = ptr::null_mut();
        env.handle_status(unsafe {
            sys::napi_create_double(env.as_sys_env(), value, &mut sys_value)
        })?;

        Ok(Self { value: sys_value })
    }
}

impl NapiValue for NapiNumber {
    fn as_sys_value(&self) -> sys::napi_value {
        self.value
    }

    fn from_sys_value(value: sys::napi_value) -> Self {
        Self { value }
    }
}
