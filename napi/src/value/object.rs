use std::ptr;

use env::NapiEnv;
use result::NapiResult;
use sys;

use super::{NapiAny, NapiArray, NapiString, NapiValue, NapiValueInternal};

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
                self.env.as_sys_env(),
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

    pub fn set_property<T, U>(&self, key: &T, value: &U) -> NapiResult<()>
    where
        T: NapiValue,
        U: NapiValue,
    {
        self.env.handle_status(unsafe {
            sys::napi_set_property(
                self.env.as_sys_env(),
                self.value,
                key.as_sys_value(),
                value.as_sys_value(),
            )
        })
    }

    pub fn get_property<T>(&self, key: &T) -> NapiResult<NapiAny>
    where
        T: NapiValue,
    {
        let mut result = ptr::null_mut();

        self.env.handle_status(unsafe {
            sys::napi_get_property(
                self.env.as_sys_env(),
                self.value,
                key.as_sys_value(),
                &mut result,
            )
        })?;

        Ok(NapiAny::with_value(self.env, result))
    }

    pub fn has_property<T>(&self, key: &T) -> NapiResult<bool>
    where
        T: NapiValue,
    {
        let mut result = false;

        self.env.handle_status(unsafe {
            sys::napi_has_property(
                self.env.as_sys_env(),
                self.value,
                key.as_sys_value(),
                &mut result,
            )
        })?;

        Ok(result)
    }

    pub fn has_own_property<T>(&self, key: &T) -> NapiResult<bool>
    where
        T: NapiValue,
    {
        let mut result = false;

        self.env.handle_status(unsafe {
            sys::napi_has_own_property(
                self.env.as_sys_env(),
                self.value,
                key.as_sys_value(),
                &mut result,
            )
        })?;

        Ok(result)
    }

    pub fn del_property<T>(&self, key: &T) -> NapiResult<bool>
    where
        T: NapiValue,
    {
        let mut result = false;

        self.env.handle_status(unsafe {
            sys::napi_delete_property(
                self.env.as_sys_env(),
                self.value,
                key.as_sys_value(),
                &mut result,
            )
        })?;

        Ok(result)
    }

    pub fn set_named_property<T>(&self, name: &str, value: &T) -> NapiResult<()>
    where
        T: NapiValue,
    {
        let key = NapiString::from_str(self.env, name)?;
        self.set_property(&key, value)
    }

    pub fn get_named_property(&self, name: &str) -> NapiResult<NapiAny> {
        let key = NapiString::from_str(self.env, name)?;
        self.get_property(&key)
    }

    pub fn has_named_property(&self, name: &str) -> NapiResult<bool> {
        let key = NapiString::from_str(self.env, name)?;
        self.has_property(&key)
    }

    pub fn del_named_property(&self, name: &str) -> NapiResult<bool> {
        let key = NapiString::from_str(self.env, name)?;
        self.del_property(&key)
    }

    pub fn set_element<T>(&self, index: u32, value: &T) -> NapiResult<()>
    where
        T: NapiValue,
    {
        self.env.handle_status(unsafe {
            sys::napi_set_element(
                self.env.as_sys_env(),
                self.value,
                index,
                value.as_sys_value(),
            )
        })
    }

    pub fn get_element(&self, index: u32) -> NapiResult<NapiAny> {
        let mut result = ptr::null_mut();

        self.env.handle_status(unsafe {
            sys::napi_get_element(
                self.env.as_sys_env(),
                self.value,
                index,
                &mut result,
            )
        })?;

        Ok(NapiAny::with_value(self.env, result))
    }

    pub fn has_element(&self, index: u32) -> NapiResult<bool> {
        let mut result = false;

        self.env.handle_status(unsafe {
            sys::napi_has_element(
                self.env.as_sys_env(),
                self.value,
                index,
                &mut result,
            )
        })?;

        Ok(result)
    }

    pub fn del_element(&self, index: u32) -> NapiResult<bool> {
        let mut result = false;

        self.env.handle_status(unsafe {
            sys::napi_delete_element(
                self.env.as_sys_env(),
                self.value,
                index,
                &mut result,
            )
        })?;

        Ok(result)
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
