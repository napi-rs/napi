use std::error::Error;
use std::fmt;
use std::fmt::Display;
use std::ptr;

use env::NapiEnv;
use sys::{napi_create_error, napi_create_range_error, napi_create_type_error,
          napi_status, napi_value};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NapiErrorKind {
    InvalidArg,
    ObjectExpected,
    StringExpected,
    NameExpected,
    FunctionExpected,
    NumberExpected,
    BooleanExpected,
    ArrayExpected,
    GenericFailure,
    PendingException,
    Cancelled,
    EscapeCalledTwice,
    ApplicationError,
}

#[derive(Clone, Debug)]
pub struct NapiError {
    pub kind: NapiErrorKind,
    pub message: Option<String>,
    pub exception: Option<napi_value>,
}

pub type NapiResult<T> = Result<T, NapiError>;

impl NapiErrorKind {
    pub fn from_napi_status(status: napi_status) -> Self {
        match status {
            napi_status::napi_invalid_arg => NapiErrorKind::InvalidArg,
            napi_status::napi_object_expected => NapiErrorKind::ObjectExpected,
            napi_status::napi_string_expected => NapiErrorKind::StringExpected,
            napi_status::napi_name_expected => NapiErrorKind::NameExpected,
            napi_status::napi_function_expected => {
                NapiErrorKind::FunctionExpected
            }
            napi_status::napi_number_expected => NapiErrorKind::NumberExpected,
            napi_status::napi_boolean_expected => {
                NapiErrorKind::BooleanExpected
            }
            napi_status::napi_array_expected => NapiErrorKind::ArrayExpected,
            napi_status::napi_generic_failure => NapiErrorKind::GenericFailure,
            napi_status::napi_pending_exception => {
                NapiErrorKind::PendingException
            }
            napi_status::napi_cancelled => NapiErrorKind::Cancelled,
            napi_status::napi_escape_called_twice => {
                NapiErrorKind::EscapeCalledTwice
            }
            _ => {
                // Both situations should never happen, so just panic.
                panic!(
                    "Either the JavaScript VM returned an unknown status code, \
                     or NapiErrorKind::from_napi_status was called with \
                     napi_status::napi_ok"
                );
            }
        }
    }
}

impl Error for NapiError {
    fn description(&self) -> &str {
        match self.kind {
            NapiErrorKind::InvalidArg => "NapiError: invalid argument",
            NapiErrorKind::ObjectExpected => "NapiError: object expected",
            NapiErrorKind::StringExpected => "NapiError: string expected",
            NapiErrorKind::NameExpected => "NapiError: name expected",
            NapiErrorKind::FunctionExpected => "NapiError: function expected",
            NapiErrorKind::NumberExpected => "NapiError: number expected",
            NapiErrorKind::BooleanExpected => "NapiError: boolean argument",
            NapiErrorKind::ArrayExpected => "NapiError: array expected",
            NapiErrorKind::GenericFailure => "NapiError: generic failure",
            NapiErrorKind::PendingException => "NapiError: pending exception",
            NapiErrorKind::Cancelled => "NapiError: cancelled",
            NapiErrorKind::EscapeCalledTwice => {
                "NapiError: escape called twice"
            }
            NapiErrorKind::ApplicationError => "NapiError: application error",
        }
    }
}

impl Display for NapiError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{}", self.description())
            .and_then(|result| if let Some(ref message) = self.message {
                write!(formatter, " ({})", message)
            } else {
                Ok(result)
            })
            .and_then(|result| if self.exception.is_some() {
                write!(formatter, ", JavaScript exception attached")
            } else {
                Ok(result)
            })
    }
}

macro_rules! error_constructor {
    ($name:ident => $napi_fn_name:ident) => {
        pub fn $name(env: &NapiEnv, message: napi_value) -> NapiError {
            let mut exception = ptr::null_mut();
            let status = unsafe {
                $napi_fn_name(
                    env.as_sys_env(),
                    ptr::null_mut(),
                    message,
                    &mut exception,
                )
            };

            if let Err(error) = env.handle_status(status) {
                return error;
            }

            NapiError {
                kind: NapiErrorKind::ApplicationError,
                message: None,
                exception: Some(exception),
            }
        }
    }
}

impl NapiError {
    error_constructor!(error => napi_create_error);
    error_constructor!(type_error => napi_create_type_error);
    error_constructor!(range_error => napi_create_range_error);
}
