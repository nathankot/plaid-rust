//! The Info endpoint allows you to retrieve various account holder
//! information on file with the financial institution, including names, emails,
//! phone numbers, and addresses.
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
//! # http_stub!(StubPolicy, 200, include_str!("fixtures/post_info_success.json"));
//! # let hyper = hyper::Client::with_connector(StubPolicy::default());
//! #
//! use plaid::api::client::{ Client, Response, Payload };
//! use plaid::api::product;
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
//!   product::Info,
//!   Payload::FetchData(client, user, None)).unwrap();
//! #
//! # match response {
//! #     Response::ProductData(ref data) => {
//! #         assert_eq!("kelly.walters30@example.com".to_string(), data.info.emails[0].email);
//! #         assert_eq!("94114".to_string(), data.info.addresses[0].address.zip);
//! #         assert_eq!("4673956022".to_string(), data.info.phone_numbers[0].phone_number);
//! #     },
//! #     _ => panic!("Expected product data")
//! # };
//! # }
//! ```

use api::product::Product;
use api::account::Account;
use api::client::Payload;
use api::data::{ Address, PhoneNumber, Email };

use rustc_serialize::{ Decodable, Decoder };

/// The definition of the Info Product.
#[derive(Debug)]
pub struct Info;

/// Representation of data that is retrieved from the `Info` product.
#[derive(Debug, RustcDecodable)]
pub struct InfoData {
    /// List of accounts associated with the user. When returned from the
    /// Info endpoint it will also include account and routing numbers.
    pub accounts: Vec<Account>,
    /// Includes all user information that has been returned.
    pub info: InfoInternalData
}

/// Represents the *actual* info data from an info response.
#[derive(Debug, RustcDecodable)]
pub struct InfoInternalData {
    /// Emails associated with the user.
    pub emails: Vec<Email>,
    /// Addresses associated with the user.
    pub addresses: Vec<InfoAddress>,
    /// Phone numbers associated with the user.
    pub phone_numbers: Vec<PhoneNumber>
}

/// Representation of an address entry returned by info.
#[derive(Debug)]
pub struct InfoAddress {
    /// Whether or not this is the user's primary address
    pub primary: bool,
    /// The underlying address
    pub address: Address
}

impl Decodable for InfoAddress {

    fn decode<D: Decoder>(d: &mut D) -> Result<InfoAddress, D::Error> {
        d.read_struct("root", 2, |d| {
            let primary = try!(d.read_struct_field("primary", 0, |d| Decodable::decode(d)));
            d.read_struct_field("data", 1, |d| {
                Ok(InfoAddress {
                    primary: primary,
                    address: try!(Decodable::decode(d))
                })
            })
        })
    }

}

impl Product for Info {
    type Data = InfoData;
    fn description<'a>(&self) -> &'a str { "Info" }
    fn endpoint<'a, 'b>(&self, payload: &'b Payload) -> &'a str {
        match *payload {
            Payload::StepMFA(..) => "/info/step",
            Payload::FetchData(..) => "/info/get",
            Payload::Upgrade(..) => "/upgrade?upgrade_to=info",
            _ => "/info"
        }
    }
}
