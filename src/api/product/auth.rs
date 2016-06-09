//! Auth is a product that allows you to authorize ACH transaction from the
//! end-users account. It will return account data including account numbers
//! and routing numbers if authorization is successful.
//!
//! ## Example of data retrieval
//!
//! ```
//! # #[macro_use(http_stub)] extern crate plaid;
//! # #[macro_use] extern crate yup_hyper_mock as hyper_mock;
//! # extern crate hyper;
//! #
//! # fn main() {
//! #
//! # http_stub!(StubPolicy, 200, include_str!("fixtures/post_auth_success.json"));
//! #
//! # let hyper = hyper::Client::with_connector(StubPolicy::default());
//! #
//! use plaid::api::client::{ Client, Response, Payload };
//! use plaid::api::product;
//! use plaid::api::types::*;
//! use plaid::api::user::{ User };
//!
//! let client = Client { endpoint:  "https://tartan.plaid.com",
//!                       client_id: "testclient",
//!                       secret:    "testsecret",
//!                       hyper:     &hyper };
//!
//! let user = User { access_token: "testaccesstoken".to_string() };
//!
//! let response = client.request(
//!   product::Auth,
//!   Payload::FetchData(client, user, None))
//!   .unwrap();
//!
//! match response {
//!     Response::ProductData(ref data) => {
//!         assert_eq!(data.accounts[0].current_balance, 1274.93 as Amount);
//!         assert_eq!(data.accounts[0].available_balance, Some(1203.42 as Amount));
//!         assert_eq!(data.accounts[0].account_number, Some("9900009606".to_string()));
//!     },
//!     _ => panic!("Expected product data")
//! };
//! # }
//! ```

use api::product::{ Product };
use api::account::Account;
use api::client::Payload;

/// `Auth` is the endpoint you need in order to check that the user owns their account.
#[derive(Debug)]
pub struct Auth;

/// Representation of data that is retrieved from the `Auth` product.
#[derive(Debug, RustcDecodable)]
pub struct AuthData {
    /// List of accounts associated with the user
    pub accounts: Vec<Account>,
}

impl Product for Auth {
    type Data = AuthData;
    fn description<'a>(&self) -> &'a str { "Auth" }
    fn endpoint<'a, 'b>(&self, payload: &'b Payload) -> &'a str {
        match *payload {
            Payload::StepMFA(..) => "/auth/step",
            Payload::FetchData(..) => "/auth/get",
            Payload::Upgrade(..) => "/upgrade?upgrade_to=auth",
            _ => "/auth"
        }
    }
}
