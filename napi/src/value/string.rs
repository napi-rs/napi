use std::ptr;

use env::NapiEnv;
use result::NapiResult;
use sys;

use super::{AsNapiObject, NapiValue, NapiValueInternal};

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

    fn to_vec<T, U>(
        &self,
        get_value: unsafe extern "C" fn(
            sys::napi_env,
            sys::napi_value,
            *mut U,
            usize,
            *mut usize,
        )
            -> sys::napi_status,
    ) -> NapiResult<Vec<T>>
    where
        T: Default + Copy,
        U: Copy,
    {
        let mut bufsize = 0;

        self.env.handle_status(unsafe {
            get_value(
                self.env.as_sys_env(),
                self.value,
                ptr::null_mut(),
                0,
                &mut bufsize,
            )
        })?;

        let mut buffer = vec![T::default(); bufsize + 1];

        self.env.handle_status(unsafe {
            get_value(
                self.env.as_sys_env(),
                self.value,
                buffer.as_mut_ptr() as *mut U,
                bufsize + 1,
                ptr::null_mut(),
            )
        })?;

        buffer.pop();

        Ok(buffer)
    }

    pub fn to_bytes(&self) -> NapiResult<Vec<u8>> {
        self.to_vec::<_, i8>(sys::napi_get_value_string_utf8)
    }

    pub fn to_latin1(&self) -> NapiResult<Vec<u8>> {
        self.to_vec::<_, i8>(sys::napi_get_value_string_latin1)
    }

    pub fn to_utf16(&self) -> NapiResult<Vec<u16>> {
        self.to_vec::<_, u16>(sys::napi_get_value_string_utf16)
    }

    pub fn to_string(&self) -> NapiResult<String> {
        let bytes = self.to_bytes()?;
        Ok(unsafe { String::from_utf8_unchecked(bytes) })
    }
}

impl<'a> NapiValue<'a> for NapiString<'a> {
    fn as_sys_value(&self) -> sys::napi_value {
        self.value
    }

    fn env(&self) -> &'a NapiEnv {
        self.env
    }
}

impl<'a> NapiValueInternal<'a> for NapiString<'a> {
    fn construct(env: &'a NapiEnv, value: sys::napi_value) -> Self {
        Self { env, value }
    }
}

impl<'a> AsNapiObject<'a> for NapiString<'a> {}
