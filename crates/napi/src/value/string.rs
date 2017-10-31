use std::ptr;

use env::NapiEnv;
use result::NapiResult;
use sys;

use super::NapiValue;

#[derive(Clone, Copy, Debug)]
pub struct NapiString<'a> {
    value: sys::napi_value,
    env: &'a NapiEnv,
}

impl<'a> NapiString<'a> {
    pub fn from_str(env: &'a NapiEnv, value: &str) -> NapiResult<Self> {
        let mut sys_value = ptr::null_mut();
        env.handle_status(unsafe {
            sys::napi_create_string_utf8(
                env.as_sys_env(),
                value.as_ptr() as *const i8,
                value.as_bytes().len(),
                &mut sys_value,
            )
        })?;

        Ok(Self {
            value: sys_value,
            env,
        })
    }

    pub fn from_latin1(env: &'a NapiEnv, value: &[u8]) -> NapiResult<Self> {
        let mut sys_value = ptr::null_mut();
        env.handle_status(unsafe {
            sys::napi_create_string_latin1(
                env.as_sys_env(),
                value.as_ptr() as *const i8,
                value.len(),
                &mut sys_value,
            )
        })?;

        Ok(Self {
            value: sys_value,
            env,
        })
    }

    pub fn from_utf16(env: &'a NapiEnv, value: &[u16]) -> NapiResult<Self> {
        let mut sys_value = ptr::null_mut();
        env.handle_status(unsafe {
            sys::napi_create_string_utf16(
                env.as_sys_env(),
                value.as_ptr(),
                value.len(),
                &mut sys_value,
            )
        })?;

        Ok(Self {
            value: sys_value,
            env,
        })
    }
}

impl<'a> NapiValue for NapiString<'a> {
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
) -> NapiString<'a> {
    NapiString { value, env }
}
