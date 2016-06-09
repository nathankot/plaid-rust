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
//!   product::Info,
//!   Payload::FetchData(client, user, None))
//!   .unwrap();
//! #
//! # match response {
//! #     Response::ProductData(ref data) => {
//! #         assert_eq!("kelly.walters30@example.com".to_string(), data.info.emails[0].email);
//! #         assert_eq!("94114".to_string(), data.info.addresses[0].zip);
//! #         assert_eq!("4673956022".to_string(), data.info.phone_numbers[0].phone_number);
//! #     },
//! #     _ => panic!("Expected product data")
//! # };
//! # }
//! ```

use api::product::Product;
use api::account::Account;
use api::client::Payload;

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
    pub addresses: Vec<Address>,
    /// Phone numbers associated with the user.
    pub phone_numbers: Vec<PhoneNumber>
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

#[derive(Debug)]
/// A user's email, including meta data returned by Plaid.
pub struct Email {
    /// Whether or not the user has chosen this as their primary email.
    pub primary: bool,
    /// The designated type for this email (e.g personal, home).
    pub email_type: String,
    /// The actual email address.
    pub email: String
}

impl Decodable for Email {

    fn decode<D: Decoder>(d: &mut D) -> Result<Email, D::Error> {
        d.read_struct("root", 3, |d| {
            Ok(Email {
                primary: try!(d.read_struct_field("primary", 0, |d| d.read_bool())),
                email_type: try!(d.read_struct_field("type", 1, |d| d.read_str())),
                email: try!(d.read_struct_field("data", 2, |d| d.read_str()))
            })
        })
    }

}

#[derive(Debug)]
/// A user's address, as returned by Plaid.
pub struct Address {
    /// Whether or not the user has chosen this as their primary address
    pub primary: bool,
    /// The address zip code part.
    pub zip: String,
    /// The address state part.
    pub state: String,
    /// The address city part.
    pub city: String,
    /// The address street part.
    pub street: String
}

impl Decodable for Address {

    fn decode<D: Decoder>(d: &mut D) -> Result<Address, D::Error> {
        d.read_struct("root", 2, |d| {
            let primary = try!(d.read_struct_field("primary", 0, |d| d.read_bool()));
            d.read_struct_field("data", 1, |d| {
                d.read_struct("address", 4, |d| {
                    Ok(Address {
                        primary: primary,
                        zip: try!(d.read_struct_field("zip", 0, |d| d.read_str())),
                        state: try!(d.read_struct_field("state", 0, |d| d.read_str())),
                        city: try!(d.read_struct_field("city", 0, |d| d.read_str())),
                        street: try!(d.read_struct_field("street", 0, |d| d.read_str()))
                    })
                })
            })
        })
    }

}

#[derive(Debug)]
/// A user's phone number, as returned by Plaid.
pub struct PhoneNumber {
    /// Whether or not the user has chosen this as their primary phone number.
    pub primary: bool,
    /// The type of the phone number (e.g personal, home).
    pub phone_number_type: String,
    /// The actual phone number.
    pub phone_number: String
}

impl Decodable for PhoneNumber {

    fn decode<D: Decoder>(d: &mut D) -> Result<PhoneNumber, D::Error> {
        d.read_struct("root", 3, |d| {
            Ok(PhoneNumber {
                primary: try!(d.read_struct_field("primary", 0, |d| d.read_bool())),
                phone_number_type: try!(d.read_struct_field("type", 1, |d| d.read_str())),
                phone_number: try!(d.read_struct_field("data", 2, |d| d.read_str()))
            })
        })
    }

}

#[cfg(test)]
mod tests {

    use api::product::info::*;
    use rustc_serialize::json;

    #[test]
    fn decode_email_works() {
        let x: Email = json::decode(r##"
            { "primary": true,
              "type": "personal",
              "data": "kelly.walters30@example.com" }
        "##).unwrap();

        assert_eq!(true, x.primary);
        assert_eq!("personal".to_string(), x.email_type);
        assert_eq!("kelly.walters30@example.com".to_string(), x.email);
    }

    #[test]
    fn decode_address_works() {
        let x: Address = json::decode(r##"
            { "primary": true,
               "data": {
                 "zip": "94114",
                 "state": "CA",
                 "city": "San Francisco",
                 "street": "3819 Greenhaven Ln"
               }
            }
        "##).unwrap();

        assert_eq!(true, x.primary);
        assert_eq!("94114".to_string(), x.zip);
        assert_eq!("CA".to_string(), x.state);
        assert_eq!("San Francisco".to_string(), x.city);
        assert_eq!("3819 Greenhaven Ln".to_string(), x.street);
    }

    #[test]
    fn decode_phone_number_works() {
        let x: PhoneNumber = json::decode(r##"
            {
                "primary": true,
                "type": "home",
                "data": "4673956022"
            }
        "##).unwrap();

        assert_eq!(true, x.primary);
        assert_eq!("home".to_string(), x.phone_number_type);
        assert_eq!("4673956022".to_string(), x.phone_number);
    }

}
