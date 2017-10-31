use std::ptr;

use env::NapiEnv;
use result::NapiResult;
use sys;

use super::NapiValue;

#[derive(Clone, Copy, Debug)]
pub struct NapiBoolean<'a> {
    value: sys::napi_value,
    env: &'a NapiEnv,
}

impl<'a> NapiBoolean<'a> {
    fn new(env: &'a NapiEnv, value: bool) -> NapiResult<Self> {
        let mut sys_value = ptr::null_mut();
        env.handle_status(unsafe {
            sys::napi_get_boolean(env.as_sys_env(), value, &mut sys_value)
        })?;

        Ok(Self {
            value: sys_value,
            env,
        })
    }

    pub fn truth(env: &'a NapiEnv) -> NapiResult<Self> {
        NapiBoolean::new(env, true)
    }

    pub fn lie(env: &'a NapiEnv) -> NapiResult<Self> {
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

impl<'a> NapiValue for NapiBoolean<'a> {
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
) -> NapiBoolean<'a> {
    NapiBoolean { value, env }
}
