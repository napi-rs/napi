use std::ptr;

use env::NapiEnv;
use result::NapiResult;
use sys;

use super::NapiValue;

#[derive(Clone, Copy, Debug)]
pub struct NapiNumber<'a> {
    value: sys::napi_value,
    env: &'a NapiEnv,
}

impl<'a> NapiNumber<'a> {
    pub fn from_i32(env: &'a NapiEnv, value: i32) -> NapiResult<Self> {
        let mut sys_value = ptr::null_mut();
        env.handle_status(unsafe {
            sys::napi_create_int32(env.as_sys_env(), value, &mut sys_value)
        })?;

        Ok(Self {
            value: sys_value,
            env,
        })
    }

    pub fn from_i64(env: &'a NapiEnv, value: i64) -> NapiResult<Self> {
        let mut sys_value = ptr::null_mut();
        env.handle_status(unsafe {
            sys::napi_create_int64(env.as_sys_env(), value, &mut sys_value)
        })?;

        Ok(Self {
            value: sys_value,
            env,
        })
    }

    pub fn from_f64(env: &'a NapiEnv, value: f64) -> NapiResult<Self> {
        let mut sys_value = ptr::null_mut();
        env.handle_status(unsafe {
            sys::napi_create_double(env.as_sys_env(), value, &mut sys_value)
        })?;

        Ok(Self {
            value: sys_value,
            env,
        })
    }
}

impl<'a> NapiValue for NapiNumber<'a> {
    fn as_sys_value(&self) -> sys::napi_value {
        self.value
    }

    fn env(&self) -> &NapiEnv {
        self.env
    }
}

pub fn construct<'a>(
    value: sys::napi_value,
    env: &'a NapiEnv,
) -> NapiNumber<'a> {
    NapiNumber { value, env }
}
