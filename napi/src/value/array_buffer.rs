use std::ptr;
use std::slice;

use env::NapiEnv;
use result::NapiResult;
use sys;

use super::{AsNapiObject, NapiValue};

#[derive(Debug)]
pub struct NapiArrayBuffer<'env, 'buf> {
    value: sys::napi_value,
    data: &'buf mut [u8],
    env: &'env NapiEnv,
}

impl<'env, 'buf> NapiArrayBuffer<'env, 'buf> {
    pub fn new(env: &'env NapiEnv, len: usize) -> NapiResult<Self> {
        let mut value = ptr::null_mut();
        let mut data = ptr::null_mut();

        env.handle_status(unsafe {
            sys::napi_create_arraybuffer(
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

impl<'env, 'buf> NapiValue<'env> for NapiArrayBuffer<'env, 'buf> {
    fn as_sys_value(&self) -> sys::napi_value {
        self.value
    }

    fn env(&self) -> &'env NapiEnv {
        self.env
    }
}

impl<'env, 'buf> AsNapiObject<'env> for NapiArrayBuffer<'env, 'buf> {}

impl<'env, 'buf> AsRef<[u8]> for NapiArrayBuffer<'env, 'buf> {
    fn as_ref(&self) -> &[u8] {
        self.data
    }
}

impl<'env, 'buf> AsMut<[u8]> for NapiArrayBuffer<'env, 'buf> {
    fn as_mut(&mut self) -> &mut [u8] {
        self.data
    }
}
