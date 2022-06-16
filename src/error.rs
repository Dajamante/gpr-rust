use std::fmt;
use std::fmt::Display;
use std::ffi::IntoStringError;
use thiserror::Error as ThisError;
use serde_json::Error as JsonError;

#[derive(Debug, Display)]
pub enum Code {
    InvalidRequest,
    CallError,
    IOError,
    UnknownError,
}

#[derive(Debug, ThisError)]
pub enum Error {
    #[error("gpr ({code}) {name}: {message}")]
    Gpr{
        code: Code,
        name: String,
        message: String
    },
    #[error(transparent)]
    Io {
        #[from]
        source: std::io::Error
    },
    #[error(transparent)]
    Cstring {
        #[from]
        source: std::ffi::IntoStringError
    },
    #[error(transparent)]
    Json {
        #[from]
        source: JsonError
    }
}

impl Error {
    pub fn from_code(code: Code, name: &str, message: &str) -> Error {
        Error::Gpr {
            code: code,
            name: String::from(name),
            message: String::from(message),
        }
    }

    pub fn from_status(status: i32, name: &str, message: &str) -> Option<Error> {
        match status {
            0 => None,
            1 => Some(Error::from_code(Code::InvalidRequest, &name, &message)),
            2 => Some(Error::from_code(Code::CallError, &name, &message)),
            3 => Some(Error::from_code(Code::UnknownError, &name, &message)),
            _ => Some(Error::from_code(
                Code::UnknownError,
                "UnknownError",
                "unknown error",
            )),
        }
    }
}
