#![deny(missing_docs)]

//! [Plaid][Plaid] is the technology layer for financial services.
//! This library makes it easy to interface with Plaid.
//!
//! In order to make calls to the API, you'll be using
//! [`Client`][Client] as the dispatcher.
//!
//! Plaid's API is grouped into different products, which are represented types that
//! implement the [`Product`][Product] trait.
//! Most of these products have the same authentication mechanism and data retrieval style.
//! Therefore an API request is built by combining a product with a [`Payload`][Payload].
//!
//! # Overview
//!
//! The following provides a high-level outline of the core components in this library:
//!
//! + **[`Client`][Client]** represents the configuration of your API credentials and
//!
//! + **[`User`][User]** represents an end-user that has authenticated their bank account,
//!   and of which you have a Plaid access token for.
//! + **[`Product`][Product]** is a trait that provides the bare minimum definition of a product
//!   (e.g Connect, Auth, Income) required in order for the [`Client`][Client] to infer
//!   request/response behavior with.
//!
//! *See the [data][data] module for a complete list.*
//!
//! # Products
//!
//! Below are the currently supported Plaid products. Each product has its own
//! module, and is documented with usage examples.
//!
//! + [Connect][connect]
//! + [Auth][auth]
//! + [Balance][balance]
//!
//! # Quick Start
//!
//! Add `plaid` as a dependency to `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! plaid = "0.1"
//! ```
//!
//! Write some code, notice that it uses [`Hyper`][Hyper] under the hood:
//!
//! ```
//! # #[macro_use(http_stub)] extern crate plaid;
//! # #[macro_use] extern crate yup_hyper_mock as hyper_mock;
//! # extern crate hyper;
//! #
//! # fn main() {
//! #
//! # http_stub!(StubPolicy, 201, include_str!("fixtures/post_connect_mfa_code.json"));
//! #
//! let hyper = hyper::client::Client::new();
//! # let hyper = hyper::Client::with_connector(StubPolicy::default());
//!
//! use plaid::api::product;
//! use plaid::api::client::{ Client, Payload };
//! # use plaid::api::types::*;
//! # use plaid::api::client::Response;
//!
//! // Build a client given your current credentials.
//! let client = Client {
//!     endpoint: "https://tartan.plaid.com",
//!     client_id: "yourclientid",
//!     secret: "yourclientsecret",
//!     hyper: &hyper
//! };
//!
//! // Authenticate the user for Plaid Connect.
//! // `response` will be `Authenticated(..)` if successful, which includes a `User`
//! let response = client.request(product::Connect,
//!     Payload::Authenticate(client,
//!                           "Chase".to_string(),
//!                           "username".to_string(),
//!                           "password".to_string(),
//!                           None,
//!                           None));
//! #
//! # match response.unwrap() {
//! #     Response::MFA(ref user, ref challenge) => {
//! #         assert_eq!(user.access_token, "test".to_string());
//! #         assert_eq!(format!("{:?}", challenge), "Code");
//! #     },
//! #     _ => panic!("Unexpected response")
//! # };
//! # }
//! ```
//!
//! Respond to multifactor authentication challenges:
//!
//! ```
//! # #[macro_use(http_stub)] extern crate plaid;
//! # #[macro_use] extern crate yup_hyper_mock as hyper_mock;
//! # extern crate hyper;
//! #
//! # fn main() {
//! #
//! # http_stub!(StubPolicy, 200, include_str!("fixtures/post_connect_success.json"));
//! #
//! let hyper = hyper::client::Client::new();
//! # let hyper = hyper::Client::with_connector(StubPolicy::default());
//!
//! use plaid::api::product;
//! use plaid::api::user::User;
//! use plaid::api::client::{ Client, Payload };
//! use plaid::api::mfa;
//! # use plaid::api::types::*;
//! # use plaid::api::client::Response;
//! #
//! # let client = Client {
//! #     endpoint: "https://tartan.plaid.com",
//! #     client_id: "yourclientid",
//! #     secret: "yourclientsecret",
//! #     hyper: &hyper
//! # };
//!
//! let user = User { access_token: "useraccesstoken".to_string() };
//! let response = client.request(
//!     product::Connect,
//!     Payload::StepMFA(client, user, mfa::Response::Code("1234".to_string())));
//! #
//! # match response.unwrap() {
//! #     Response::Authenticated(ref user, ref data) => {
//! #         assert_eq!(user.access_token, "test".to_string());
//! #         assert_eq!(data.accounts[0].current_balance, 742.93 as Amount);
//! #         assert_eq!(data.accounts[1].current_balance, 100030.32 as Amount);
//! #         assert_eq!(data.transactions[0].amount, -700 as Amount);
//! #         assert_eq!(data.transactions[1].id, "testtransactionid2".to_string());
//! #     },
//! #     _ => panic!("Unexpected response")
//! # };
//! # }
//! ```
//!
//! Fetch data from the product:
//!
//! ```
//! # #[macro_use(http_stub)] extern crate plaid;
//! # #[macro_use] extern crate yup_hyper_mock as hyper_mock;
//! # extern crate hyper;
//! #
//! # fn main() {
//! #
//! # http_stub!(StubPolicy, 200, include_str!("fixtures/post_connect_success.json"));
//! #
//! let hyper = hyper::client::Client::new();
//! # let hyper = hyper::Client::with_connector(StubPolicy::default());
//!
//! use plaid::api::product;
//! use plaid::api::user::User;
//! use plaid::api::client::{ Client, Payload };
//! use plaid::api::client::payload::FetchDataOptions;
//! # use plaid::api::types::*;
//! # use plaid::api::client::Response;
//! #
//! # let client = Client {
//! #     endpoint: "https://tartan.plaid.com",
//! #     client_id: "yourclientid",
//! #     secret: "yourclientsecret",
//! #     hyper: &hyper
//! # };
//!
//! let user = User { access_token: "useraccesstoken".to_string() };
//! let response = client.request(
//!     product::Connect,
//!     Payload::FetchData(client, user, Some(FetchDataOptions::default())));
//! #
//! # match response.unwrap() {
//! #     Response::ProductData(ref data) => {
//! #         assert_eq!(data.accounts[0].current_balance, 742.93 as Amount);
//! #         assert_eq!(data.accounts[1].current_balance, 100030.32 as Amount);
//! #         assert_eq!(data.transactions[0].amount, -700 as Amount);
//! #         assert_eq!(data.transactions[1].id, "testtransactionid2".to_string());
//! #     },
//! #     _ => panic!("Unexpected response")
//! # };
//! # }
//! ```
//!
//! [Plaid]: https://www.plaid.com
//! [Hyper]: http://hyper.rs
//!
//! [User]: ./api/user/struct.User.html
//! [Product]: ./api/product/trait.Product.html
//! [Payload]: ./api/client/payload/enum.Payload.html
//! [Client]: ./api/client/struct.Client.html
//! [client]: ./api/client/index.html
//! [data]: ./api/data/index.html
//!
//! [connect]: ./api/product/connect/index.html
//! [auth]: ./api/product/auth/index.html
//! [balance]: ./api/product/balance/index.html

extern crate rustc_serialize;
extern crate hyper;

pub mod api;

/// A helper for testing against stubbed API responses.
#[doc(hidden)] #[macro_export]
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
