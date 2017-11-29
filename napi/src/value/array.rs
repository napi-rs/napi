use std::ptr;

use env::NapiEnv;
use result::{NapiError, NapiResult};
use sys;

use super::{AsNapiObject, NapiAny, NapiString, NapiValue, NapiValueInternal};

#[derive(Clone, Copy, Debug)]
pub struct NapiArray<'env> {
    value: sys::napi_value,
    env: &'env NapiEnv,
}

impl<'env> NapiArray<'env> {
    pub fn new(env: &'env NapiEnv) -> NapiResult<Self> {
        let mut value = ptr::null_mut();
        env.handle_status(unsafe {
            sys::napi_create_array(env.as_sys_env(), &mut value)
        })?;

        Ok(Self { value, env })
    }

    pub fn with_len(env: &'env NapiEnv, len: usize) -> NapiResult<Self> {
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

    pub fn get(&self, index: u32) -> NapiResult<NapiAny<'env>> {
        self.as_napi_object().get_element(index)
    }

    pub fn set<T>(&self, index: u32, value: &T) -> NapiResult<()>
    where
        T: NapiValue<'env>,
    {
        self.as_napi_object().set_element(index, value)
    }
}

impl<'env> NapiValue<'env> for NapiArray<'env> {
    fn as_sys_value(&self) -> sys::napi_value {
        self.value
    }

    fn env(&self) -> &'env NapiEnv {
        self.env
    }

    fn from_sys_checked(
        env: &'env NapiEnv,
        value: sys::napi_value,
    ) -> NapiResult<Self> {
        if !NapiAny::with_value(env, value).is_array()? {
            let message = NapiString::from_str(env, "Array expected")?;
            return Err(NapiError::type_error(env, &message));
        }

        Ok(Self { env, value })
    }
}

impl<'env> NapiValueInternal<'env> for NapiArray<'env> {
    fn construct(env: &'env NapiEnv, value: sys::napi_value) -> Self {
        Self { env, value }
    }
}

impl<'env> AsNapiObject<'env> for NapiArray<'env> {}
