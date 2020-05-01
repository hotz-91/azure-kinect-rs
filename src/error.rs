use super::bindings::*;

#[derive(Clone, Copy, Debug)]
pub enum Error {
    Succeded,
    Failed,
    TooSmall,
    Timeout,
    Win32Error(u32),
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

pub(crate) trait ToResult: Sized {
    fn to_result(&self) -> Result<Self, Error>;
}

impl From<k4a_result_t> for Error {
    fn from(s: k4a_result_t) -> Error {
        match s {
            K4A_RESULT_SUCCEEDED => Error::Succeded,
            K4A_RESULT_FAILED => Error::Failed,
        }
    }
}

impl From<k4a_buffer_result_t> for Error {
    fn from(s: k4a_buffer_result_t) -> Error {
        match s {
            K4A_RESULT_SUCCEEDED => Error::Succeded,
            K4A_RESULT_FAILED => Error::Failed,
            K4A_BUFFER_RESULT_TOO_SMALL => Error::TooSmall,
        }
    }
}

impl From<k4a_wait_result_t> for Error {
    fn from(s: k4a_wait_result_t) -> Error {
        match s {
            K4A_RESULT_SUCCEEDED => Error::Succeded,
            K4A_RESULT_FAILED => Error::Failed,
            K4A_WAIT_RESULT_TIMEOUT => Error::Timeout,
        }
    }
}
