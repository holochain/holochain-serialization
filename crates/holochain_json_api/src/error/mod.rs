//! This module contains Error type definitions that are used throughout Holochain, and the Ribosome in particular,
//! which is responsible for mounting and running instances of DNA, and executing WASM code.

use self::JsonError::*;
use crate::json::*;
use futures::channel::oneshot::Canceled as FutureCanceled;
use serde_json::Error as SerdeError;
use std::{
    error::Error,
    fmt,
    io::{self, Error as IoError},
    option::NoneError,
};

//--------------------------------------------------------------------------------------------------
// JsonError
//--------------------------------------------------------------------------------------------------

#[derive(
    Clone, Debug, PartialEq, Eq, Serialize, Deserialize, DefaultJson, Hash, PartialOrd, Ord,
)]
pub enum JsonError {
    ErrorGeneric(String),
    IoError(String),
    SerializationError(String),
}

impl JsonError {
    pub fn new(msg: &str) -> JsonError {
        JsonError::ErrorGeneric(msg.to_string())
    }
}

pub type JsonResult<T> = Result<T, JsonError>;

impl fmt::Display for JsonError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ErrorGeneric(err_msg) => write!(f, "{}", err_msg),
            SerializationError(err_msg) => write!(f, "{}", err_msg),
            IoError(err_msg) => write!(f, "{}", err_msg),
        }
    }
}

impl Error for JsonError {}

impl From<JsonError> for String {
    fn from(holochain_json_error: JsonError) -> Self {
        holochain_json_error.to_string()
    }
}

impl From<String> for JsonError {
    fn from(error: String) -> Self {
        JsonError::new(&error)
    }
}

impl From<&'static str> for JsonError {
    fn from(error: &str) -> Self {
        JsonError::new(error)
    }
}

/// standard strings for std io errors
fn reason_for_io_error(error: &IoError) -> String {
    match error.kind() {
        io::ErrorKind::InvalidData => format!("contains invalid data: {}", error),
        io::ErrorKind::PermissionDenied => format!("missing permissions to read: {}", error),
        _ => format!("unexpected error: {}", error),
    }
}

impl<T> From<::std::sync::PoisonError<T>> for JsonError {
    fn from(error: ::std::sync::PoisonError<T>) -> Self {
        JsonError::ErrorGeneric(format!("sync poison error: {}", error))
    }
}

impl From<IoError> for JsonError {
    fn from(error: IoError) -> Self {
        JsonError::IoError(reason_for_io_error(&error))
    }
}

impl From<SerdeError> for JsonError {
    fn from(error: SerdeError) -> Self {
        JsonError::SerializationError(error.to_string())
    }
}

impl From<base64::DecodeError> for JsonError {
    fn from(error: base64::DecodeError) -> Self {
        JsonError::ErrorGeneric(format!("base64 decode error: {}", error.to_string()))
    }
}

impl From<std::str::Utf8Error> for JsonError {
    fn from(error: std::str::Utf8Error) -> Self {
        JsonError::ErrorGeneric(format!("std::str::Utf8Error error: {}", error.to_string()))
    }
}

impl From<FutureCanceled> for JsonError {
    fn from(_: FutureCanceled) -> Self {
        JsonError::ErrorGeneric("Failed future".to_string())
    }
}

impl From<NoneError> for JsonError {
    fn from(_: NoneError) -> Self {
        JsonError::ErrorGeneric("Expected Some and got None".to_string())
    }
}

impl From<hcid::HcidError> for JsonError {
    fn from(error: hcid::HcidError) -> Self {
        JsonError::ErrorGeneric(format!("{:?}", error))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // a test function that returns our error result
    fn raises_json_error(yes: bool) -> JsonResult<()> {
        if yes {
            Err(JsonError::new("borked"))
        } else {
            Ok(())
        }
    }

    #[test]
    /// test that we can convert an error to a string
    fn to_string() {
        let err = JsonError::new("foo");
        assert_eq!("foo", err.to_string());
    }

    #[test]
    /// test that we can convert an error to valid JSON
    fn test_to_json() {
        let err = JsonError::new("foo");
        assert_eq!(
            JsonString::from_json("{\"ErrorGeneric\":\"foo\"}"),
            JsonString::from(err),
        );
    }

    #[test]
    /// smoke test new errors
    fn can_instantiate() {
        let err = JsonError::new("borked");

        assert_eq!(JsonError::ErrorGeneric("borked".to_string()), err);
    }

    #[test]
    /// test errors as a result and destructuring
    fn can_raise_json_error() {
        let err = raises_json_error(true).expect_err("should return an error when yes=true");

        match err {
            JsonError::ErrorGeneric(msg) => assert_eq!(msg, "borked"),
            _ => panic!("raises_json_error should return an ErrorGeneric"),
        };
    }

    #[test]
    /// test errors as a returned result
    fn can_return_result() {
        let result = raises_json_error(false);

        assert!(result.is_ok());
    }

    #[test]
    /// show Error implementation for JsonError
    fn error_test() {
        for (input, output) in vec![
            (JsonError::ErrorGeneric(String::from("foo")), "foo"),
            (JsonError::SerializationError(String::from("foo")), "foo"),
            (JsonError::IoError(String::from("foo")), "foo"),
        ] {
            assert_eq!(output, &input.to_string());
        }
    }

}
