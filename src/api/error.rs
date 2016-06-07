//! The error module

use api::product::{Product};
use std::error::Error as StdError;
use std::io::Error as IOError;
use std::fmt;
use std::fmt::Debug;
use std::any::Any;
use hyper;
use rustc_serialize::json::{DecoderError, EncoderError};

/// # Error
/// Represents possible errors returned from the API.
/// `D` is the `Data` associated type in `Product`.
#[derive(Debug)]
pub enum Error<'a, Data: 'a> {
    /// Returned when a request is made for a `Product` that is not
    /// currently enabled for the given `User`.
    ///
    /// If this occurs, you should upgrade the `User` so that they have
    /// access to the `Product`.
    ProductNotEnabled(&'a Product<Data=Data>),
    /// Represents errors forwarded from `rustc_serialize`, usually indicating
    /// that the response returned something that could not be decoded, or that
    /// the response returned an HTTP failure.
    InvalidResponse(DecoderError),
    /// Represents an error forwarded from `hyper`, which means it is most
    /// likely HTTP related.
    HTTP(hyper::Error),
    /// Returned for errors that are forwarded from `std::io::Error`
    IO(IOError),
    /// This should happen very rarely, and indicates that something is most
    /// likely wrong with `plaid::api` rather than the end user.
    InternalError,
}

impl<'a, Data: Debug + Any> fmt::Display for Error<'a, Data> {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.description())
    }

}

impl<'a, Data: Debug + Any> StdError for Error<'a, Data> {

    fn description(&self) -> &str {
        match *self {
            Error::ProductNotEnabled(ref p) => p.description(),
            Error::InvalidResponse(ref err) => err.description(),
            Error::HTTP(ref err) => err.description(),
            Error::IO(ref err) => err.description(),
            Error::InternalError => "`plaid::api` internal error, please contact Plaid for support",
        }
    }

}

impl<'a, Data> From<IOError> for Error<'a, Data> {

    fn from(err: IOError) -> Error<'a, Data> {
        Error::IO(err)
    }

}

impl<'a, Data> From<hyper::Error> for Error<'a, Data> {

    fn from(err: hyper::Error) -> Error<'a, Data> {
        Error::HTTP(err)
    }

}

impl<'a, Data> From<DecoderError> for Error<'a, Data> {

    fn from(err: DecoderError) -> Error<'a, Data> {
        Error::InvalidResponse(err)
    }

}

impl<'a, Data> From<EncoderError> for Error<'a, Data> {

    fn from(err: EncoderError) -> Error<'a, Data> {
        Error::InternalError
    }

}
