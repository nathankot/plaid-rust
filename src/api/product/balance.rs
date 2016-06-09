//! Balance is a product that allows users to query the account balance
//! of a given user.
//!
//! ## Endpoint example
//!
//! ```
//! # #[macro_use(http_stub)] extern crate plaid;
//! # #[macro_use] extern crate yup_hyper_mock as hyper_mock;
//! # extern crate hyper;
//! #
//! # fn main() {
//! #
//! # http_stub!(StubPolicy, 200, include_str!("fixtures/post_balance_success.json"));
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
//!   product::Balance,
//!   Payload::FetchData(client, user, None))
//!   .unwrap();
//!
//! match response {
//!     Response::ProductData(ref data) => {
//!         assert_eq!(data.accounts[0].current_balance, 742.93 as Amount);
//!         assert_eq!(data.accounts[1].current_balance, 100030.32 as Amount);
//!     },
//!     _ => panic!("Expected product data")
//! };
//! # }
//! ```

use api::product::{ Product };
use api::account::Account;
use api::client::Payload;

/// `Balance` is the endpoint you need to fetch transaction for a `User`
#[derive(Debug)]
pub struct Balance;

/// Representation of data that is retrieved from the `Balance` product.
#[derive(Debug, RustcDecodable)]
pub struct BalanceData {
    /// List of accounts associated with the user
    pub accounts: Vec<Account>
}

impl Product for Balance {
    type Data = BalanceData;
    fn description<'a>(&self) -> &'a str { "Balance" }
    fn endpoint<'a, 'b>(&self, payload: &'b Payload) -> &'a str {
        match *payload {
            Payload::StepMFA(..) => "/balance/step",
            Payload::FetchData(..) => "/balance/get",
            Payload::Upgrade(..) => "/upgrade?upgrade_to=balance",
            _ => "/balance"
        }
    }
}
