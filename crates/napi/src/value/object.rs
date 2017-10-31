use std::ptr;

use env::NapiEnv;
use result::NapiResult;
use sys;

use super::{NapiAny, NapiArray, NapiValue, NapiValueInternal};

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

    pub fn prototype(&self) -> NapiResult<NapiAny> {
        let mut result = ptr::null_mut();

        self.env.handle_status(unsafe {
            sys::napi_get_prototype(
                self.env().as_sys_env(),
                self.value,
                &mut result,
            )
        })?;

        Ok(NapiAny::with_value(self.env(), result))
    }

    pub fn property_names(&self) -> NapiResult<NapiArray> {
        let mut result = ptr::null_mut();

        self.env.handle_status(unsafe {
            sys::napi_get_property_names(
                self.env.as_sys_env(),
                self.value,
                &mut result,
            )
        })?;

        Ok(NapiArray::construct(self.env, result))
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

impl<'a> NapiValueInternal<'a> for NapiObject<'a> {
    fn construct(env: &'a NapiEnv, value: sys::napi_value) -> Self {
        Self { env, value }
    }
}
