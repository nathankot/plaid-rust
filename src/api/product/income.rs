//! The Income endpoint allows you to retrieve various information pertaining to a user's income.
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
//! # http_stub!(StubPolicy, 200, include_str!("fixtures/post_income_success.json"));
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
//!   product::Income,
//!   Payload::FetchData(client, user, None))
//!   .unwrap();
//! #
//! # match response {
//! #     Response::ProductData(ref data) => {
//! #         assert_eq!(data.income.income_streams[0].monthly_income, 5250 as Amount);
//! #         assert_eq!(data.income.income_streams[0].confidence, 1.0 as f64);
//! #         assert_eq!(data.income.income_streams[0].days, 284 as u64);
//! #     },
//! #     _ => panic!("Expected product data")
//! # };
//! # }
//! ```

use api::product::{ Product };
use api::client::Payload;
use api::data::{ Amount, Account };

use rustc_serialize::{ Decoder };

#[derive(Debug)]
/// The Income endpoint.
pub struct Income;

#[derive(Debug, RustcDecodable)]
/// The underlying data representation of Income.
pub struct IncomeData {
    /// A list of user accounts and their balances.
    pub accounts: Vec<Account>,
    /// Income data
    pub income: IncomeInternalData
}

impl Product for Income {
    type Data = IncomeData;
    fn description<'a>(&self) -> &'a str { "Income" }
    fn endpoint<'a, 'b>(&self, payload: &'b Payload) -> &'a str {
        match *payload {
            Payload::StepMFA(..) => "/income/step",
            Payload::FetchData(..) => "/income/get",
            Payload::Upgrade(..) => "/upgrade?upgrade_to=income",
            _ => "/income"
        }
    }
}

#[derive(Debug, RustcDecodable)]
/// Internal data representation of the income response
pub struct IncomeInternalData {
    /// A list of income streams.
    pub income_streams: Vec<IncomeStream>,
    /// The sum of user's income over the past 365 days.
    /// If we have less than 365 days of data this will be less than a full year's income.
    pub last_year_income: Amount,
    /// `last_year_income` interpolated to value before taxes.
    /// This is the minimum pre-tax salary that assumes a filing status of
    /// single with zero dependents.
    pub last_year_income_before_tax: Amount,
    /// User's income extrapolated over a year based on current,
    /// active income streams. Income streams become inactive if they have not
    /// recurred for more than two cycles. For example, if a weekly paycheck hasn't
    /// been seen for the past two weeks, it is no longer active.
    pub projected_yearly_income: Amount,
    /// `projected_yearly_income` interpolated to value before taxes.
    /// This is the minimum pre-tax salary that assumes a filing status of
    /// single with zero dependents.
    pub projected_yearly_income_before_tax: Amount,
    /// Max number of income streams present at the same time over the past 365 days.
    pub max_number_of_overlapping_income_streams: u64,
    /// Total number of distinct income streams received over the past 365 days.
    pub number_of_income_streams: u64
}

#[derive(Debug, RustcDecodable)]
/// An income stream represents a stream of income that Plaid
/// has detected from their transactions.
pub struct IncomeStream {
    /// How much income per month in dollars.
    pub monthly_income: Amount,
    /// Plaid's confidence in this estimate.
    pub confidence: f64,
    /// The number of days Plaid has seen this for.
    pub days: u64,
    /// The name of the income stream.
    pub name: String
}
