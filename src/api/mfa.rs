//! This module contains data structures and implementations
//! related to multi-factor-authentication.

use api::user;
use api::client;
use api::product::*;

use rustc_serialize::{Decodable, Decoder};

/// Used to provide an alternative decode method
/// when there is an MFA challenge.
pub struct User<P: Product>(pub user::User<P>);

/// Represents one of the different types of multi-factor-authentication
/// challenges Plaid supports.
///
/// Todo: support all mfa challenges
#[derive(Debug, Eq, PartialEq)]
pub enum Challenge {
    /// A token-based authorization, this token will be sent to one of
    /// the user's registered devices.
    Code
}

/// Represents a response to a previously given MFA challenge.
#[derive(Debug, Eq, PartialEq)]
pub enum Response {
    /// A response to a code challenge, providing the code
    /// that was sent to the user's device.
    Code(String)
}

impl<'a, P: Product> Decodable for User<P> {

    fn decode<D: Decoder>(decoder: &mut D) -> Result<User<P>, D::Error> {
        decoder.read_struct("root", 0, |decoder| {
            let access_token = try!(decoder.read_struct_field("access_token", 0, |d| Decodable::decode(d)));
            let challenge_type = try!(decoder.read_struct_field("type", 0, |d| {
                let t: String = try!(Decodable::decode(d));
                match t.as_ref() {
                    "device" => Ok(Challenge::Code),
                    _ => Err(d.error("Un-supported mfa preference"))
                }
            }));

            Ok(User(user::User {
                access_token: access_token,
                status: client::Response::MFA(challenge_type)
            }))
        })
    }

}

// /// An intermediate type used to build a request to the
// /// `/step` endpoint.
// struct ResponseRequest<'a, P: Product> {
//     client: Client<'a>>,
//     user: User<P>,>
//     response: Response
// }

// impl<'a, P: Product> Encodable for ResponseRequest<'a, P> {

//     fn encode<S: Encoder>(&self, encoder: &mut S) -> Result<(), S::Error> {
//         encoder.emit_struct("Request", 5, |encoder| {
//             try!(encoder.emit_struct_field("client_id", 0, |e| self.client.client_id.encode(e)));
//             try!(encoder.emit_struct_field("secret", 1, |e| self.client.secret.encode(e)));
//             try!(encoder.emit_struct_field("username", 2, |e| self.username.encode(e)));
//             try!(encoder.emit_struct_field("password", 3, |e| self.password.encode(e)));
//             try!(encoder.emit_struct_field("type", 4, |e| self.institution.encode(e)));
//             Ok(())
//         })
//     }

// }
