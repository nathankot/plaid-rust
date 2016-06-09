//! Connect is a product that Plaid offers. It allows you to retrieve account balance
//! and transaction history data.
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
//! # http_stub!(StubPolicy, 200, include_str!("fixtures/post_connect_success.json"));
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
//!   product::Connect,
//!   Payload::FetchData(client, user, None))
//!   .unwrap();
//!
//! match response {
//!     Response::ProductData(ref data) => {
//!         assert_eq!(data.accounts[0].current_balance, 742.93 as Amount);
//!         assert_eq!(data.accounts[1].current_balance, 100030.32 as Amount);
//!         assert_eq!(data.transactions[0].amount, -700 as Amount);
//!         assert_eq!(data.transactions[1].id, "testtransactionid2".to_string());
//!     },
//!     _ => panic!("Expected product data")
//! };
//! # }
//! ```

use api::product::{ Product };
use api::account::Account;
use api::transaction::Transaction;
use api::client::Payload;

/// `Connect` is the endpoint you need to fetch transaction for a `User`
#[derive(Debug)]
pub struct Connect;

/// Representation of data that is retrieved from the `Connect` product.
#[derive(Debug, RustcDecodable)]
pub struct ConnectData {
    /// List of accounts associated with the user
    pub accounts: Vec<Account>,
    /// List of transactions associated with the user
    pub transactions: Vec<Transaction>
}

impl Product for Connect {
    type Data = ConnectData;
    fn description<'a>(&self) -> &'a str { "Connect" }
    fn endpoint<'a, 'b>(&self, payload: &'b Payload) -> &'a str {
        match *payload {
            Payload::StepMFA(..) => "/connect/step",
            Payload::FetchData(..) => "/connect/get",
            Payload::Upgrade(..) => "/upgrade?upgrade_to=connect",
            _ => "/connect"
        }
    }
}
