use std::mem;

use env::NapiEnv;
use result::NapiResult;
use sys;

pub trait NapiValue {
    fn as_sys_value(&self) -> sys::napi_value;
}

#[derive(Clone, Copy, Debug)]
pub struct NapiUndefined {
    value: sys::napi_value,
}

impl NapiUndefined {
    pub fn new(env: &NapiEnv) -> NapiResult<Self> {
        let sys_env: sys::napi_env = env.as_sys_env();

        let value = unsafe {
            let mut result = mem::uninitialized();
            let status = sys::napi_get_undefined(sys_env, &mut result);
            env.handle_status(status)?;
            result
        };

        Ok(Self { value })
    }
}

impl NapiValue for NapiUndefined {
    fn as_sys_value(&self) -> sys::napi_value {
        self.value
    }
}
