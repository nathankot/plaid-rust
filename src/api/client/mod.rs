//! Client

use std::io::Read;

use api::user::User;
use api::product::*;
use api::error::Error;
use api::mfa;

use rustc_serialize::json;

use hyper as h;
use hyper::header::{ContentType, Accept, qitem};
use hyper::mime::{Mime, TopLevel, SubLevel, Attr, Value};
use hyper::status::StatusCode;

pub use self::payload::Payload;
pub mod payload;

pub use self::response::Response;
pub mod response;

/// # Client
///
/// Represents a Plaid API consumer. Encapsulates the `endpoint`,
/// `client_id` and `secret` of the consumer.
///
/// This is where all requests to the API start.
#[derive(Copy, Clone)]
pub struct Client<'a> {
    /// E.g `https://api.plaid.com`
    pub endpoint: &'a str,
    /// Your application's `client_id`
    pub client_id: &'a str,
    /// Your application's `secret`
    pub secret: &'a str,
    /// The instance of `hyper::Client` to use.
    /// *In most cases* you simply need `hyper::Client::new()`.
    /// However this is a good place to configure things like
    /// proxies, timeouts etc.
    pub hyper: &'a h::Client
}

impl<'a> Client<'a> {

    /// Create a new `User` using their provided credentials and institution.
    /// You only need to call this once for a given user. There-after you should
    /// store the provided `access_token` and the authenticated `Product` for future
    /// reference.
    ///
    /// # Examples
    ///
    /// ## A successful authorization that requires a MFA step
    ///
    /// ```
    /// # #[macro_use(http_stub)] extern crate plaid;
    /// # #[macro_use] extern crate yup_hyper_mock as hyper_mock;
    /// # extern crate hyper;
    /// #
    /// # fn main() {
    /// #
    /// # http_stub!(StubPolicy, 201, include_str!("fixtures/post_connect_mfa_code.json"));
    /// #
    /// # let hyper = hyper::Client::with_connector(StubPolicy::default());
    /// #
    /// use plaid::api::client;
    /// use plaid::api::product;
    /// use plaid::api::client::payload::Payload;
    /// use plaid::api::client::response::Response;
    ///
    /// let client = client::Client { endpoint:  "https://tartan.plaid.com",
    ///                               client_id: "testclient",
    ///                               secret:    "testsecret",
    ///                               hyper:     &hyper };
    ///
    /// let response = client.request(
    ///   product::Connect,
    ///   Payload::Authenticate(
    ///      client,
    ///      "case".to_string(),
    ///      "username".to_string(),
    ///      "password".to_string(),
    ///      None,
    ///      None))
    ///   .unwrap();
    ///
    /// match response {
    ///     Response::MFA(ref user, ref challenge) => {
    ///         assert_eq!(user.access_token, "test".to_string());
    ///         assert_eq!(format!("{:?}", challenge), "Code");
    ///     },
    ///     _ => panic!("Unexpected response")
    /// };
    /// # }
    /// ```
    ///
    /// ## A successful authorization without an MFA step
    ///
    /// ```
    /// # #[macro_use(http_stub)] extern crate plaid;
    /// # #[macro_use] extern crate yup_hyper_mock as hyper_mock;
    /// # extern crate hyper;
    /// #
    /// # fn main() {
    /// #
    /// # http_stub!(StubPolicy, 200, include_str!("fixtures/post_connect_success.json"));
    /// #
    /// # let hyper = hyper::Client::with_connector(StubPolicy::default());
    /// #
    /// use plaid::api::client::{ Client, Response, Payload };
    /// use plaid::api::product;
    /// use plaid::api::types::*;
    ///
    /// let client = Client { endpoint:  "https://tartan.plaid.com",
    ///                       client_id: "testclient",
    ///                       secret:    "testsecret",
    ///                       hyper:     &hyper };
    ///
    /// let response = client.request(
    ///   product::Connect,
    ///   Payload::Authenticate(
    ///       client,
    ///       "chase".to_string(),
    ///       "username".to_string(),
    ///       "password".to_string(),
    ///       None,
    ///       None))
    ///   .unwrap();
    ///
    /// match response {
    ///     Response::Authenticated(ref user, ref data) => {
    ///         assert_eq!(user.access_token, "test".to_string());
    ///         assert_eq!(data.accounts[0].current_balance, 742.93 as Amount);
    ///         assert_eq!(data.accounts[1].current_balance, 100030.32 as Amount);
    ///         assert_eq!(data.transactions[0].amount, -700 as Amount);
    ///         assert_eq!(data.transactions[1].id, "testtransactionid2".to_string());
    ///     },
    ///     _ => panic!("Expected product data")
    /// };
    /// # }
    /// ```
    ///
    /// ## A successful product data retrieval
    ///
    /// ```
    /// # #[macro_use(http_stub)] extern crate plaid;
    /// # #[macro_use] extern crate yup_hyper_mock as hyper_mock;
    /// # extern crate hyper;
    /// #
    /// # fn main() {
    /// #
    /// # http_stub!(StubPolicy, 200, include_str!("fixtures/post_connect_success.json"));
    /// #
    /// # let hyper = hyper::Client::with_connector(StubPolicy::default());
    /// #
    /// use plaid::api::client::{ Client, Response, Payload };
    /// use plaid::api::product;
    /// use plaid::api::types::*;
    /// use plaid::api::user::{ User };
    ///
    /// let client = Client { endpoint:  "https://tartan.plaid.com",
    ///                       client_id: "testclient",
    ///                       secret:    "testsecret",
    ///                       hyper:     &hyper };
    ///
    /// let user = User { access_token: "testaccesstoken".to_string() };
    ///
    /// let response = client.request(
    ///   product::Connect,
    ///   Payload::FetchData(client, user, None))
    ///   .unwrap();
    ///
    /// match response {
    ///     Response::ProductData(ref data) => {
    ///         assert_eq!(data.accounts[0].current_balance, 742.93 as Amount);
    ///         assert_eq!(data.accounts[1].current_balance, 100030.32 as Amount);
    ///         assert_eq!(data.transactions[0].amount, -700 as Amount);
    ///         assert_eq!(data.transactions[1].id, "testtransactionid2".to_string());
    ///     },
    ///     _ => panic!("Expected product data")
    /// };
    /// # }
    /// ```
    pub fn request<P: Product>(&self, product: P, payload: Payload) -> Result<Response<P>, Error> {

        let body = try!(json::encode(&payload));
        let mut body = body.into_bytes();
        let body_capacity = body.len();
        let endpoint = payload.endpoint(&self, product);
        let method = payload.method();

        let mut res: h::client::Response = try!(
            self.hyper
                .request(method, endpoint.as_ref() as &str)
                .header(ContentType(Mime(TopLevel::Application, SubLevel::Json, vec![])))
                .header(Accept(vec![qitem(Mime(TopLevel::Application, SubLevel::Json,
                                               vec![(Attr::Charset, Value::Utf8)]))]))
                .body(h::client::Body::BufBody(&mut body, body_capacity))
                .send());

        let mut buffer = String::new();
        match (res.status, payload) {
            // A `201` indicates that the `User` has been created but
            // is missing the multi-factor authentication step.
            (StatusCode::Created, _) => {
                try!(res.read_to_string(&mut buffer));
                let user: User = try!(json::decode(&mut buffer));
                let mfa_challenge: mfa::Challenge = try!(json::decode(&mut buffer));
                Ok(Response::MFA(user, mfa_challenge))
            },
            // A `200` response for authentication is accompanied with the
            // endpoint data that was requested for.
            (StatusCode::Ok, Payload::Authenticate( .. )) |
            (StatusCode::Ok, Payload::StepMFA( .. )) => {
                try!(res.read_to_string(&mut buffer));
                let mut buffer_copy = buffer.clone();
                let user: User = try!(json::decode(&mut buffer));
                let data: P::Data = try!(json::decode(&mut buffer_copy));
                Ok(Response::Authenticated(user, data))
            },
            // A `200` response for data requests
            (StatusCode::Ok, Payload::FetchData( .. )) => {
                try!(res.read_to_string(&mut buffer));
                let mut buffer_copy = buffer.clone();
                let data: P::Data = try!(json::decode(&mut buffer_copy));
                Ok(Response::ProductData(data))
            },
            // By default, we assume a bad response
            (ref s, _) => return Err(Error::UnsuccessfulResponse(*s))
        }

    }

}
