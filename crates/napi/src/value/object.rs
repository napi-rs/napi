use std::ptr;

use env::NapiEnv;
use result::NapiResult;
use sys;

use super::NapiValue;

#[derive(Clone, Copy, Debug)]
pub struct NapiObject<'a> {
    value: sys::napi_value,
    env: &'a NapiEnv,
}

impl<'a> NapiObject<'a> {
    pub fn new(env: &'a NapiEnv) -> NapiResult<Self> {
        let mut value = ptr::null_mut();
        env.handle_status(unsafe {
            sys::napi_create_object(env.as_sys_env(), &mut value)
        })?;

        Ok(Self { value, env })
    }
}

impl<'a> NapiValue for NapiObject<'a> {
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
) -> NapiObject<'a> {
    NapiObject { value, env }
}
