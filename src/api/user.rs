//! User module

use super::product::*; //, Auth, Info, Income, Risk };
use super::client::{ Client };
use super::error::Error;
use super::types::*;
use super::mfa::{ MFAChallenge, MFAChallengedUser };

use std::io::Read;
use std::result::{ Result };

use rustc_serialize::{Decodable, Decoder, Encodable, Encoder};
use rustc_serialize::json;

/// # User
/// Represents an authorized user for a given product.
#[derive(Debug)]
pub struct User<P: Product> {
    /// The result of the previous api request that returned this `Struct`
    pub status: Status<P>,
    /// The access token for this user
    pub access_token: AccessToken
}

/// # Status
/// Represents the status of the last API request for this user.
/// This does not encapsulate any errors, rather it indicates different
/// stages of the user lifecycle.
#[derive(Debug)]
pub enum Status<P: Product> {
    /// Nothing is known about the user and no requests have been made
    Unknown,
    /// Waiting on MFA authentication code from the user
    MFAChallenged(MFAChallenge),
    /// Returned when a request is made for a `Product` that is not
    /// currently enabled for the given `User`.
    ///
    /// If this occurs, you should upgrade the `User` so that they have
    /// access to the `Product`.
    ProductNotEnabled(P),
    /// User is authenticated successfully and we have data available
    Success
}

use hyper as h;
use hyper::header::{ContentType, Accept, ContentLength, qitem};
use hyper::mime::{Mime, TopLevel, SubLevel, Attr, Value};
use hyper::status::StatusCode;

impl<P: Product> User<P> {

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
    /// http_stub!(StubPolicy, 201, r##"
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
    /// use plaid::api::user::{User, Status };
    /// use plaid::api::mfa::{ MFAChallenge, MFAChallengedUser };
    ///
    /// let client = client::Client { endpoint:  "https://tartan.plaid.com",
    ///                               client_id: "testclient",
    ///                               secret:    "testsecret" };
    ///
    /// let user = User::create(
    ///   client,
    ///   product::Connect,
    ///   "chase".to_string(),
    ///   "username".to_string(),
    ///   "password".to_string(),
    ///   hyper).unwrap();
    ///
    /// assert_eq!(user.access_token, "test".to_string());
    /// assert_eq!(format!("{:?}", user.status), "MFAChallenged(Code)");
    /// # }
    /// ```
    ///
    /// Todo: allow options and passing webhooks
    pub fn create<'a>(
        client: Client<'a>,
        product: P,
        institution: Institution,
        username: Username,
        password: Password,
        hyper: h::Client) -> Result<Self, Error> {

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

        match res.status {
            // A `201` indicates that the `User` has been created but
            // is missing the multi-factor authentication step.
            StatusCode::Created => {
                try!(res.read_to_string(&mut buffer));
                let user: MFAChallengedUser<P> = try!(json::decode(&mut buffer));
                let MFAChallengedUser(u): MFAChallengedUser<P> = user;
                Ok(u) as Result<Self, Error>
            },
            // All okay, we have
            StatusCode::Ok => {
                try!(res.read_to_string(&mut buffer));
                let user: User<P> = try!(json::decode(&mut buffer));
                Ok(user)
            },
            // By default, we assume a bad response
            ref s => return Err(Error::BadResponse(*s))
        }

    }

}

/// Represents a request for creating a new user
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

impl<'a, P: Product> Decodable for User<P> {

    fn decode<D: Decoder>(decoder: &mut D) -> Result<User<P>, D::Error> {
        decoder.read_struct("root", 3, |decoder| {
            let access_token = try!(decoder.read_struct_field("access_token", 0, |d| Decodable::decode(d)));

            let status = {
                Status::Unknown
            };

            Ok(User {
                access_token: access_token,
                status: status
            })
        })
    }

}
