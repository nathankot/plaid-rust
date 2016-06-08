//! Payload

use api::types::*;
use api::product::*;
use api::user::User;
use api::client::Client;
use api::mfa;

use rustc_serialize::{Decodable, Decoder, Encodable, Encoder};
use rustc_serialize::json;

use hyper::method::Method;

/// Use this enum to tell the client what you want to do
/// with the associated product.
/// Todo: allow options and passing webhooks
pub enum Payload<'a> {
    /// Authenticate a user.
    Authenticate(Client<'a>, Institution, Username, Password),
    /// Send multifactor authentication response.
    StepMFA(Client<'a>, User, mfa::Response),
    /// Retrieve data from the product
    FetchData(Client<'a>, User)
}

impl<'a> Payload<'a> {

    /// Returns the desired endpoint of the payload, given a `Product`
    pub fn endpoint<P: Product>(&self, client: &'a Client<'a>, product: P) -> String {
        format!("{}{}", client.endpoint, product.endpoint(&self))
    }

    /// Returns the `hyper::method::Method` to be used for the request
    pub fn method(&self) -> Method {
        match *self {
            Payload::Authenticate(..) => Method::Post,
            Payload::StepMFA(..) => Method::Patch,
            Payload::FetchData(..) => Method::Get
        }
    }

}

impl<'a> Encodable for Payload<'a> {


    fn encode<S: Encoder>(&self, encoder: &mut S) -> Result<(), S::Error> {
        match *self {
            Payload::Authenticate(ref client, ref institution, ref username, ref password) => {
                encoder.emit_struct("Request", 5, |encoder| {
                    try!(encoder.emit_struct_field("client_id", 0, |e| client.client_id.encode(e)));
                    try!(encoder.emit_struct_field("secret", 1, |e| client.secret.encode(e)));
                    try!(encoder.emit_struct_field("username", 2, |e| username.encode(e)));
                    try!(encoder.emit_struct_field("password", 3, |e| password.encode(e)));
                    try!(encoder.emit_struct_field("type", 4, |e| institution.encode(e)));
                    Ok(())
                })
            },
            Payload::StepMFA(ref client, ref user, ref mfa_response) => {
                encoder.emit_struct("Request", 4, |encoder| {
                    try!(encoder.emit_struct_field("client_id", 0, |e| client.client_id.encode(e)));
                    try!(encoder.emit_struct_field("secret", 1, |e| client.secret.encode(e)));
                    try!(encoder.emit_struct_field("access_token", 2, |e| user.access_token.encode(e)));
                    match *mfa_response {
                        mfa::Response::Code(ref code) => {
                            try!(encoder.emit_struct_field("mfa", 3, |e| code.encode(e)))
                        }
                    }
                    Ok(())
                })
            },
            _ => unimplemented!()
        }
    }

}
