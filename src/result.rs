use std;
use std::error;
use std::fmt;

use c_api;

/// This crate specific [`Result`] type.
///
/// [`Result`]: https://doc.rust-lang.org/std/result/enum.Result.html
pub type Result<T> = std::result::Result<T, Error>;

/// Possible errors.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(missing_docs)]
pub enum Error {
    BackendNotSupported,
    EcMethodNotImplemented,
    BackendInitError,
    BackendInUse,
    BackendNotAvailable,
    BadChecksum,
    InvalidParams,
    BadHeader,
    InsufficientFragments,
    Other(u32),
}
impl Error {
    /// Makes a `Error` instance from the given codepoint defined in [openstack/liberasurecode].
    ///
    /// [openstack/liberasurecode]: https://github.com/openstack/liberasurecode
    pub fn from_error_code(code: u32) -> Self {
        match code {
            c_api::EBACKENDNOTSUPP => Error::BackendNotSupported,
            c_api::EECMETHODNOTIMPL => Error::EcMethodNotImplemented,
            c_api::EBACKENDINITERR => Error::BackendInitError,
            c_api::EBACKENDINUSE => Error::BackendInUse,
            c_api::EBACKENDNOTAVAIL => Error::BackendNotAvailable,
            c_api::EBADCHKSUM => Error::BadChecksum,
            c_api::EINVALIDPARAMS => Error::InvalidParams,
            c_api::EBADHEADER => Error::BadHeader,
            c_api::EINSUFFFRAGS => Error::InsufficientFragments,
            _ => Error::Other(code),
        }
    }

    /// Returns the codepoint of the error.
    pub fn as_error_code(self) -> u32 {
        match self {
            Error::BackendNotSupported => c_api::EBACKENDNOTSUPP,
            Error::EcMethodNotImplemented => c_api::EECMETHODNOTIMPL,
            Error::BackendInitError => c_api::EBACKENDINITERR,
            Error::BackendInUse => c_api::EBACKENDINUSE,
            Error::BackendNotAvailable => c_api::EBACKENDNOTAVAIL,
            Error::BadChecksum => c_api::EBADCHKSUM,
            Error::InvalidParams => c_api::EINVALIDPARAMS,
            Error::BadHeader => c_api::EBADHEADER,
            Error::InsufficientFragments => c_api::EINSUFFFRAGS,
            Error::Other(code) => code,
        }
    }
}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::BackendNotSupported => write!(f, "The backend is not supported"),
            Error::EcMethodNotImplemented => {
                write!(f, "The erasure coding method is not implemented")
            }
            Error::BackendInitError => write!(f, "Initialization of the backend is failed"),
            Error::BackendInUse => write!(f, "The backend is in use"),
            Error::BackendNotAvailable => write!(f, "The backend is not available"),
            Error::BadChecksum => write!(f, "Bad checksum value"),
            Error::InvalidParams => write!(f, "Invalid parameters"),
            Error::BadHeader => write!(f, "Bad header"),
            Error::InsufficientFragments => write!(f, "Insufficient fragments"),
            Error::Other(code) => write!(f, "Unknown error (code={})", code),
        }
    }
}
impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::BackendNotSupported => "The backend is not supported",
            Error::EcMethodNotImplemented => "The erasure coding method is not implemented",
            Error::BackendInitError => "Initialization of the backend is failed",
            Error::BackendInUse => "The backend is in use",
            Error::BackendNotAvailable => "The backend is not available",
            Error::BadChecksum => "Bad checksum value",
            Error::InvalidParams => "Invalid parameters",
            Error::BadHeader => "Bad header",
            Error::InsufficientFragments => "Insufficient fragments",
            Error::Other(_) => "Unknown error",
        }
    }
    fn cause(&self) -> Option<&dyn error::Error> {
        None
    }
}
