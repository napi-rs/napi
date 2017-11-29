use std::ptr;
use std::slice;

use env::NapiEnv;
use result::{NapiError, NapiResult};
use sys;

use super::{AsNapiObject, NapiAny, NapiString, NapiValue};

#[derive(Debug)]
pub struct NapiBuffer<'env, 'buf> {
    value: sys::napi_value,
    data: &'buf mut [u8],
    env: &'env NapiEnv,
}

impl<'env, 'buf> NapiBuffer<'env, 'buf> {
    pub fn new(env: &'env NapiEnv, len: usize) -> NapiResult<Self> {
        let mut value = ptr::null_mut();
        let mut data = ptr::null_mut();

        env.handle_status(unsafe {
            sys::napi_create_buffer(
                env.as_sys_env(),
                len,
                &mut data,
                &mut value,
            )
        })?;

        Ok(Self {
            value,
            data: unsafe { slice::from_raw_parts_mut(data as *mut u8, len) },
            env,
        })
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

impl<'env, 'buf> NapiValue<'env> for NapiBuffer<'env, 'buf> {
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
        if !NapiAny::with_value(env, value).is_buffer()? {
            let message = NapiString::from_str(env, "Buffer expected")?;
            return Err(NapiError::type_error(env, &message));
        }

        let mut data = ptr::null_mut();
        let mut len = 0;

        env.handle_status(unsafe {
            sys::napi_get_buffer_info(
                env.as_sys_env(),
                value,
                &mut data,
                &mut len,
            )
        })?;

        Ok(Self {
            env,
            value,
            data: unsafe { slice::from_raw_parts_mut(data as *mut u8, len) },
        })
    }
}

impl<'env, 'buf> AsNapiObject<'env> for NapiBuffer<'env, 'buf> {}

impl<'env, 'buf> AsRef<[u8]> for NapiBuffer<'env, 'buf> {
    fn as_ref(&self) -> &[u8] {
        self.data
    }
}

impl<'env, 'buf> AsMut<[u8]> for NapiBuffer<'env, 'buf> {
    fn as_mut(&mut self) -> &mut [u8] {
        self.data
    }
}
