//! Namespace for error definitions. Responses are only considered to
//! be errors if they fall outside of the expected user flow. By that
//! definition, all non 2XX HTTP response codes are considered an error.

use std::error::Error as StdError;
use std::io::Error as IOError;
use std::fmt;
use hyper;
use rustc_serialize::json::{DecoderError, EncoderError};

/// # Error
/// Represents possible errors returned from the API.
/// `P` represents the product that the error is scoped for.
#[derive(Debug)]
pub enum Error {
    /// Represents bad HTTP status codes, or codes that we don't support.
    UnsuccessfulResponse(hyper::status::StatusCode),
    /// Represents errors forwarded from `rustc_serialize`, usually indicating
    /// that the response returned something that could not be decoded.
    InvalidResponse(DecoderError),
    /// Represents an error forwarded from `hyper`, which means it is most
    /// likely HTTP (protocol, rather than status code) related.
    HTTP(hyper::Error),
    /// Returned for errors that are forwarded from `std::io::Error`
    IO(IOError),
    /// This should happen very rarely, and indicates that something is most
    /// likely wrong with `plaid::api` rather than the end user.
    InternalError,
}

impl fmt::Display for Error {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.description())
    }

}

impl StdError for Error {

    fn description(&self) -> &str {
        match *self {
            Error::UnsuccessfulResponse(_) => "Received bad status code",
            Error::InvalidResponse(ref err) => err.description(),
            Error::HTTP(ref err) => err.description(),
            Error::IO(ref err) => err.description(),
            Error::InternalError => "`plaid::api` internal error, please contact Plaid for support",
        }
    }

}

impl From<IOError> for Error {

    fn from(err: IOError) -> Error {
        Error::IO(err)
    }

}

impl From<hyper::Error> for Error {

    fn from(err: hyper::Error) -> Error {
        Error::HTTP(err)
    }

}

impl From<DecoderError> for Error {

    fn from(err: DecoderError) -> Error {
        Error::InvalidResponse(err)
    }

}

impl From<EncoderError> for Error {

    fn from(_: EncoderError) -> Error {
        Error::InternalError
    }

}
