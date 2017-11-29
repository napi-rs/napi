use std::ptr;

use env::NapiEnv;
use result::{NapiError, NapiResult};
use sys;

use super::{NapiArray, NapiBoolean, NapiNull, NapiNumber, NapiObject,
            NapiString, NapiUndefined, NapiValue, NapiValueInternal,
            NapiValueType};

#[derive(Clone, Copy, Debug)]
pub struct NapiAny<'env> {
    value: sys::napi_value,
    env: &'env NapiEnv,
}

impl<'env> NapiAny<'env> {
    pub fn new(env: &'env NapiEnv) -> NapiResult<Self> {
        let mut value = ptr::null_mut();
        env.handle_status(unsafe {
            sys::napi_get_undefined(env.as_sys_env(), &mut value)
        })?;

        Ok(Self { value, env })
    }

    pub fn with_value(env: &'env NapiEnv, value: sys::napi_value) -> Self {
        Self { env, value }
    }

    pub fn as_undefined(&self) -> NapiResult<NapiUndefined<'env>> {
        match self.value_type()? {
            NapiValueType::Undefined => {
                Ok(NapiUndefined::construct(self.env(), self.as_sys_value()))
            }
            _ => Err(NapiError::type_error(
                self.env(),
                &NapiString::from_str(self.env(), "undefined expected")?,
            )),
        }
    }

    pub fn as_null(&self) -> NapiResult<NapiNull<'env>> {
        match self.value_type()? {
            NapiValueType::Null => {
                Ok(NapiNull::construct(self.env(), self.as_sys_value()))
            }
            _ => Err(NapiError::type_error(
                self.env(),
                &NapiString::from_str(self.env(), "null expected")?,
            )),
        }
    }

    pub fn as_boolean(&self) -> NapiResult<NapiBoolean<'env>> {
        match self.value_type()? {
            NapiValueType::Boolean => {
                Ok(NapiBoolean::construct(self.env(), self.as_sys_value()))
            }
            _ => Err(NapiError::type_error(
                self.env(),
                &NapiString::from_str(self.env(), "boolean expected")?,
            )),
        }
    }

    pub fn as_number(&self) -> NapiResult<NapiNumber<'env>> {
        match self.value_type()? {
            NapiValueType::Number => {
                Ok(NapiNumber::construct(self.env(), self.as_sys_value()))
            }
            _ => Err(NapiError::type_error(
                self.env(),
                &NapiString::from_str(self.env(), "number expected")?,
            )),
        }
    }

    pub fn as_string(&self) -> NapiResult<NapiString<'env>> {
        match self.value_type()? {
            NapiValueType::String => {
                Ok(NapiString::construct(self.env(), self.as_sys_value()))
            }
            _ => Err(NapiError::type_error(
                self.env(),
                &NapiString::from_str(self.env(), "string expected")?,
            )),
        }
    }

    pub fn as_object(&self) -> NapiResult<NapiObject<'env>> {
        match self.value_type()? {
            NapiValueType::Object
            | NapiValueType::String
            | NapiValueType::Function => {
                Ok(NapiObject::construct(self.env(), self.as_sys_value()))
            }
            _ => Err(NapiError::type_error(
                self.env(),
                &NapiString::from_str(self.env(), "object expected")?,
            )),
        }
    }

    pub fn as_array(&self) -> NapiResult<NapiArray<'env>> {
        if self.is_array()? {
            Ok(NapiArray::construct(self.env(), self.as_sys_value()))
        } else {
            Err(NapiError::type_error(
                self.env(),
                &NapiString::from_str(self.env(), "array expected")?,
            ))
        }
    }
}

impl<'env> NapiValue<'env> for NapiAny<'env> {
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
        Ok(Self { env, value })
    }
}
