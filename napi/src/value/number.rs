use std::ptr;

use env::NapiEnv;
use result::NapiResult;
use sys;

use super::{NapiValue, NapiValueInternal};

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

    pub fn to_i32(&self) -> NapiResult<i32> {
        let mut result = 0;

        self.env.handle_status(unsafe {
            sys::napi_get_value_int32(
                self.env.as_sys_env(),
                self.value,
                &mut result,
            )
        })?;

        Ok(result)
    }

    pub fn to_i64(&self) -> NapiResult<i64> {
        let mut result = 0;

        self.env.handle_status(unsafe {
            sys::napi_get_value_int64(
                self.env.as_sys_env(),
                self.value,
                &mut result,
            )
        })?;

        Ok(result)
    }

    pub fn to_f64(&self) -> NapiResult<f64> {
        let mut result = 0.0;

        self.env.handle_status(unsafe {
            sys::napi_get_value_double(
                self.env.as_sys_env(),
                self.value,
                &mut result,
            )
        })?;

        Ok(result)
    }
}

impl<'a> NapiValue<'a> for NapiNumber<'a> {
    fn as_sys_value(&self) -> sys::napi_value {
        self.value
    }

    fn env(&self) -> &'a NapiEnv {
        self.env
    }
}

impl<'a> NapiValueInternal<'a> for NapiNumber<'a> {
    fn construct(env: &'a NapiEnv, value: sys::napi_value) -> Self {
        Self { env, value }
    }
}
