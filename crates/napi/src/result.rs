use std::error::Error;
use std::fmt;
use std::fmt::Display;

use sys::{napi_status, napi_value};

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
