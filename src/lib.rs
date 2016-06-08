#![deny(missing_docs)]

//! # Plaid
//!
//! Plaid is the technology layer for financial services.

extern crate rustc_serialize;
extern crate hyper;

pub mod api;

/// A helper for testing against stubbed API responses.
#[macro_export]
macro_rules! http_stub {
    ($name:ident, $status:expr, $json:expr) => (
        mock_connector!($name {
            "https://api.plaid.com" => {
            format!(r############"
HTTP/1.1 {} OK
Content-Type: application/json; charset=utf-8

{}
            "############, $status.to_string(), $json)
            }

            "https://tartan.plaid.com" => {
            format!(r############"
HTTP/1.1 {} OK
Content-Type: application/json; charset=utf-8

{}
            "############, $status.to_string(), $json)
            }
        });
    )
}
