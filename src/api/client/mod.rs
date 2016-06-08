//! Client

use std::io::Read;

use api::user::User;
use api::product::*;
use api::types::*;
use api::error::Error;
use api::mfa;

use rustc_serialize::json;

use hyper as h;
use hyper::method::Method;
use hyper::header::{ContentType, Accept, ContentLength, qitem};
use hyper::mime::{Mime, TopLevel, SubLevel, Attr, Value};
use hyper::status::StatusCode;

mod payloads;

pub use self::response::Response;
mod response;

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
    /// use plaid::api::user::{ User };
    ///
    /// let client = client::Client { endpoint:  "https://tartan.plaid.com",
    ///                               client_id: "testclient",
    ///                               secret:    "testsecret",
    ///                               hyper:     &hyper };
    ///
    /// let (user, response) = client.authenticate(
    ///   product::Connect,
    ///   "chase".to_string(),
    ///   "username".to_string(),
    ///   "password".to_string()).unwrap();
    ///
    /// assert_eq!(user.access_token, "test".to_string());
    /// assert_eq!(format!("{:?}", response), "MFA(Code)");
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
    /// use plaid::api::client::{ Client, Response };
    /// use plaid::api::product;
    /// use plaid::api::types::*;
    /// use plaid::api::user::{ User };
    ///
    /// let client = Client { endpoint:  "https://tartan.plaid.com",
    ///                       client_id: "testclient",
    ///                       secret:    "testsecret",
    ///                       hyper:     &hyper };
    ///
    /// let (user, response) = client.authenticate(
    ///   product::Connect,
    ///   "chase".to_string(),
    ///   "username".to_string(),
    ///   "password".to_string()).unwrap();
    ///
    /// assert_eq!(user.access_token, "test".to_string());
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
    ///
    /// Todo: allow options and passing webhooks
    pub fn authenticate<P: Product>(
        &self,
        product: P,
        institution: Institution,
        username: Username,
        password: Password) -> Result<(User, Response<P>), Error> {

        let req = payloads::Authenticate { client: self.clone(),
                                           username: username,
                                           password: password,
                                           institution: institution };
        let body = try!(json::encode(&req));
        let mut body = body.into_bytes();
        let body_capacity = body.len();

        let mut res = try!(
            self.request( Method::Post, product, "")
                .header(ContentLength(body_capacity as u64))
                .body(h::client::Body::BufBody(&mut body, body_capacity))
                .send());

        let mut buffer = String::new();
        match res.status {
            // A `201` indicates that the `User` has been created but
            // is missing the multi-factor authentication step.
            StatusCode::Created => {
                try!(res.read_to_string(&mut buffer));
                let mut buffer_copy = buffer.clone();
                let user: User = try!(json::decode(&mut buffer));
                let mfa_challenge: mfa::Challenge = try!(json::decode(&mut buffer));
                Ok((user, Response::MFA(mfa_challenge)))
            },
            // A `200` response is accompanied with the endpoint data that
            // was requested for.
            StatusCode::Ok => {
                try!(res.read_to_string(&mut buffer));
                let mut buffer_copy = buffer.clone();
                let user: User = try!(json::decode(&mut buffer));
                let data: P::Data = try!(json::decode(&mut buffer_copy));
                Ok((user, Response::ProductData(data)))
            },
            // By default, we assume a bad response
            ref s => return Err(Error::UnsuccessfulResponse(*s))
        }

    }

    /// Given a `User` that has received an `Response::MFA`, you
    /// can use this method to complete the `mfa::Challenge`.
    ///
    /// # Examples
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
    /// use plaid::api::client::{ Client, Response };
    /// use plaid::api::product;
    /// use plaid::api::types::*;
    /// use plaid::api::user::{ User };
    /// use plaid::api::mfa;
    ///
    /// let client = Client { endpoint:  "https://tartan.plaid.com",
    ///                       client_id: "testclient",
    ///                       secret:    "testsecret",
    ///                       hyper:     &hyper };
    ///                       
    /// let user = User { access_token: "testaccesstoken".to_string() };
    ///
    /// let response = client.step(
    ///   product::Connect,
    ///   user,
    ///   mfa::Response::Code("1234".to_string())).unwrap();
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
    pub fn step<P: Product>(
        &self,
        product: P,
        user: User,
        response: mfa::Response) -> Result<Response<P>, Error> {

        let req = payloads::MFAStep { client: self.clone(),
                                      user: user,
                                      response: response };
        let body = try!(json::encode(&req));
        let mut body = body.into_bytes();
        let body_capacity = body.len();

        let mut res = try!(
            self.request(Method::Patch, product, "/step")
                .header(ContentLength(body_capacity as u64))
                .body(h::client::Body::BufBody(&mut body, body_capacity))
                .send());

        let mut buffer = String::new();
        match res.status {
            StatusCode::Ok => {
                try!(res.read_to_string(&mut buffer));
                let data: P::Data = try!(json::decode(&mut buffer));
                Ok(Response::ProductData(data))
            },
            ref s => {
                Err(Error::UnsuccessfulResponse(*s))
            }
        }
    }

    fn request<P: Product>(
        &self,
        method: Method,
        product: P,
        component: &str) -> h::client::RequestBuilder<'a> {

        self.hyper
            .request(method,
                     &format!("{}{}{}",
                              self.endpoint,
                              product.endpoint_component(),
                              component))
            .header(ContentType(Mime(TopLevel::Application, SubLevel::Json, vec![])))
            .header(Accept(vec![qitem(Mime(TopLevel::Application, SubLevel::Json,
                        vec![(Attr::Charset, Value::Utf8)]))]))
    }

}

