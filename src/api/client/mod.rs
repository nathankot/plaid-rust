//! Data structures and methods that interact with Plaid via HTTP.

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
    /// E.g `https://api.plaid.com`.
    pub endpoint: &'a str,
    /// Your application's `client_id`.
    pub client_id: &'a str,
    /// Your application's `secret`.
    pub secret: &'a str,
    /// The instance of `hyper::Client` to use.
    /// *In most cases* you simply need `hyper::Client::new()`.
    /// However this is a good place to configure things like
    /// proxies, timeouts etc.
    pub hyper: &'a h::Client
}

impl<'a> Client<'a> {

    /// Make a request to the given [Product](../product/struct.Product.html), using a
    /// [Payload](./payload/struct.Payload.html) describing the intention of the operation.
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
