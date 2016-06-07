//! User module

use super::product::{ Product, Connect, Auth, Info, Income, Risk };
use super::client::{ Client };
use super::error::Error;
use super::types::*;

use std::io;
use std::io::Read;
use std::result::{ Result };

use std::any::Any;
use std::fmt::Debug;

use rustc_serialize::{Decodable, Decoder, Encodable, Encoder};
use rustc_serialize::json;

/// # User
/// Represents an authorized user for a given product.
#[derive(RustcDecodable, Debug)]
pub struct User {
    /// The access token for this user
    pub access_token: AccessToken
}

/// # Status
/// Represents the status of the last API request for this user.
/// This does not encapsulate any errors, rather it indicates different
/// stages of the user lifecycle.
pub enum Status<D: Debug + Any> {
    /// Nothing is known about the user and no requests have been made
    Unknown,
    /// Waiting on MFA authentication code from the user
    MFARequested,
    /// Indicates that the user needs to upgrade in order to use
    /// the previously queries `Product`
    UpgradeRequired(Product<Data=D>),
    /// We have data related to the given product
    Data
}

use hyper as h;
use hyper::header::{ContentType, Accept, ContentLength, qitem};
use hyper::mime::{Mime, TopLevel, SubLevel, Attr, Value};

impl User {

    /// Create a new `User` using their provided credentials and institution.
    /// You only need to call this once for a given user. There-after you should
    /// store the provided `access_token` and the authenticated `Product` for future
    /// reference.
    ///
    /// # Examples
    ///
    /// A successful connection that requires an MFA step:
    ///
    /// ```
    /// # #[macro_use(http_stub)] extern crate plaid;
    /// # #[macro_use] extern crate yup_hyper_mock as hyper_mock;
    /// # extern crate hyper;
    /// #
    /// # fn main() {
    /// #
    /// http_stub!(StubPolicy, 200, r##"
    ///   {
    ///     "access_token": "test",
    ///     "mfa": { "message": "Code sent to ...e@nathankot.com" },
    ///     "type": "device"
    ///   }
    /// "##);
    /// #
    /// # let hyper = hyper::Client::with_connector(StubPolicy::default());
    /// #
    /// use plaid::api::client;
    /// use plaid::api::product;
    /// use plaid::api::user;
    ///
    /// let client = client::Client { endpoint:  "https://tartan.plaid.com",
    ///                               client_id: "testclient",
    ///                               secret:    "testsecret" };
    ///
    /// let user = user::User::create(
    ///   client,
    ///   product::Connect,
    ///   "chase".to_string(),
    ///   "nathankot1".to_string(),
    ///   "password".to_string(),
    ///   hyper
    /// );
    ///
    /// assert_eq!(user.unwrap().access_token, "test".to_string());
    /// # }
    /// ```
    ///
    /// Todo: allow options and passing webhooks
    pub fn create<P: Product>(
        client: Client,
        product: P,
        institution: Institution,
        username: Username,
        password: Password,
        hyper: h::Client) -> Result<User, Error<P::Data>> {

        let mut buffer = String::new();
        let endpoint = client.endpoint;
        let req = UserCreateRequest { client: client,
                                      username: username,
                                      password: password,
                                      institution: institution };
        let body = try!(json::encode(&req));
        let mut body = body.into_bytes();
        let body_capacity = body.len();


        let mut res = try!(
            hyper.post(&format!("{}{}", endpoint, product.endpoint_component()))
                 .header(ContentType(Mime(TopLevel::Application, SubLevel::Json, vec![])))
                 .header(ContentLength(body_capacity as u64))
                 .header(Accept(vec![qitem(Mime(TopLevel::Application, SubLevel::Json,
                                vec![(Attr::Charset, Value::Utf8)]))]))
                 .body(h::client::Body::BufBody(&mut body, body_capacity))
                 .send());

        try!(res.read_to_string(&mut buffer));
        let user: User = try!(json::decode(&mut buffer));
        Ok(user)
    }

}

struct UserCreateRequest<'a> {
    client: Client<'a>,
    username: Username,
    password: Password,
    institution: Institution,
}

impl<'a> Encodable for UserCreateRequest<'a> {

    fn encode<S: Encoder>(&self, encoder: &mut S) -> Result<(), S::Error> {
        encoder.emit_struct("Request", 5, |encoder| {
            try!(encoder.emit_struct_field("client_id", 0, |e| self.client.client_id.encode(e)));
            try!(encoder.emit_struct_field("secret", 1, |e| self.client.secret.encode(e)));
            try!(encoder.emit_struct_field("username", 2, |e| self.username.encode(e)));
            try!(encoder.emit_struct_field("password", 3, |e| self.password.encode(e)));
            try!(encoder.emit_struct_field("type", 4, |e| self.institution.encode(e)));
            Ok(())
        })
    }

}
