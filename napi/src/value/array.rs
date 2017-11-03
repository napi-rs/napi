use std::ptr;

use env::NapiEnv;
use result::NapiResult;
use sys;

use super::{AsNapiObject, NapiAny, NapiValue, NapiValueInternal};

#[derive(Clone, Copy, Debug)]
pub struct NapiArray<'a> {
    value: sys::napi_value,
    env: &'a NapiEnv,
}

impl<'a> NapiArray<'a> {
    pub fn new(env: &'a NapiEnv) -> NapiResult<Self> {
        let mut value = ptr::null_mut();
        env.handle_status(unsafe {
            sys::napi_create_array(env.as_sys_env(), &mut value)
        })?;

        Ok(Self { value, env })
    }

    pub fn with_len(env: &'a NapiEnv, len: usize) -> NapiResult<Self> {
        let mut value = ptr::null_mut();
        env.handle_status(unsafe {
            sys::napi_create_array_with_length(
                env.as_sys_env(),
                len,
                &mut value,
            )
        })?;

        Ok(Self { value, env })
    }

    pub fn len(&self) -> NapiResult<u32> {
        let mut result = 0;

        self.env.handle_status(unsafe {
            sys::napi_get_array_length(
                self.env.as_sys_env(),
                self.as_sys_value(),
                &mut result,
            )
        })?;

        Ok(result)
    }

    pub fn is_empty(&self) -> NapiResult<bool> {
        self.len().map(|l| l == 0)
    }

    pub fn get(&self, index: u32) -> NapiResult<NapiAny<'a>> {
        self.as_napi_object().get_element(index)
    }

    pub fn set<T>(&self, index: u32, value: &T) -> NapiResult<()>
    where
        T: NapiValue<'a>,
    {
        self.as_napi_object().set_element(index, value)
    }
}

impl<'a> NapiValue<'a> for NapiArray<'a> {
    fn as_sys_value(&self) -> sys::napi_value {
        self.value
    }

    fn env(&self) -> &'a NapiEnv {
        self.env
    }
}

impl<'a> NapiValueInternal<'a> for NapiArray<'a> {
    fn construct(env: &'a NapiEnv, value: sys::napi_value) -> Self {
        Self { env, value }
    }
}

impl<'a> AsNapiObject<'a> for NapiArray<'a> {}
