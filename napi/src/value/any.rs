use std::ptr;

use env::NapiEnv;
use result::{NapiError, NapiResult};
use sys;

use super::{NapiArray, NapiBoolean, NapiNull, NapiNumber, NapiObject,
            NapiString, NapiUndefined, NapiValue, NapiValueInternal,
            NapiValueType};

#[derive(Clone, Copy, Debug)]
pub struct NapiAny<'a> {
    value: sys::napi_value,
    env: &'a NapiEnv,
}

impl<'a> NapiAny<'a> {
    pub fn new(env: &'a NapiEnv) -> NapiResult<Self> {
        let mut value = ptr::null_mut();
        env.handle_status(unsafe {
            sys::napi_get_undefined(env.as_sys_env(), &mut value)
        })?;

        Ok(Self { value, env })
    }

    pub fn with_value(env: &'a NapiEnv, value: sys::napi_value) -> Self {
        Self { env, value }
    }

    pub fn as_undefined(&self) -> NapiResult<NapiUndefined<'a>> {
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

    pub fn as_null(&self) -> NapiResult<NapiNull<'a>> {
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

    pub fn as_boolean(&self) -> NapiResult<NapiBoolean<'a>> {
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

    pub fn as_number(&self) -> NapiResult<NapiNumber<'a>> {
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

    pub fn as_string(&self) -> NapiResult<NapiString<'a>> {
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

    pub fn as_object(&self) -> NapiResult<NapiObject<'a>> {
        match self.value_type()? {
            NapiValueType::Object |
            NapiValueType::String |
            NapiValueType::Function => {
                Ok(NapiObject::construct(self.env(), self.as_sys_value()))
            }
            _ => Err(NapiError::type_error(
                self.env(),
                &NapiString::from_str(self.env(), "object expected")?,
            )),
        }
    }

    pub fn as_array(&self) -> NapiResult<NapiArray<'a>> {
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

impl<'a> NapiValue<'a> for NapiAny<'a> {
    fn as_sys_value(&self) -> sys::napi_value {
        self.value
    }

    fn env(&self) -> &'a NapiEnv {
        self.env
    }
}
