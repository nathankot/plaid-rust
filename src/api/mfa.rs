//! This module contains data structures and implementations
//! related to multi-factor-authentication.

use super::user::{ Status, User };
use super::product::*;

use rustc_serialize::{Decodable, Decoder};

/// Used internally to provide an alternative decode method
/// when there is an MFA challenge.
pub struct MFAChallengedUser<P: Product>(pub User<P>);

/// Represents one of the different types of multi-factor-authentication
/// challenges Plaid supports.
///
/// Todo: support all mfa challenges
#[derive(Debug, Eq, PartialEq)]
pub enum MFAChallenge {
    /// A token-based authorization, this token will be sent to one of
    /// the user's registered devices.
    Code
}

impl<'a, P: Product> Decodable for MFAChallengedUser<P> {

    fn decode<D: Decoder>(decoder: &mut D) -> Result<MFAChallengedUser<P>, D::Error> {
        decoder.read_struct("root", 0, |decoder| {
            let access_token = try!(decoder.read_struct_field("access_token", 0, |d| Decodable::decode(d)));
            let challenge_type = try!(decoder.read_struct_field("type", 0, |d| {
                let t: String = try!(Decodable::decode(d));
                match t.as_ref() {
                    "device" => Ok(MFAChallenge::Code),
                    _ => Err(d.error("Un-supported mfa preference"))
                }
            }));

            Ok(MFAChallengedUser(User {
                access_token: access_token,
                status: Status::MFAChallenged(challenge_type)
            }))
        })
    }

}
