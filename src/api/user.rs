//! User module

use api::product::*;
use api::client::{ Response };
use api::types::*;

use rustc_serialize::{Decodable, Decoder };

/// # User
/// Represents an authorized user for a given product.
#[derive(Debug)]
pub struct User {
    /// The access token for this user
    pub access_token: AccessToken
}

impl Decodable for User {

    fn decode<D: Decoder>(decoder: &mut D) -> Result<User, D::Error> {
        decoder.read_struct("root", 3, |decoder| {
            Ok(User {
                access_token: try!(decoder.read_struct_field("access_token", 0, |d| Decodable::decode(d)))
            })
        })
    }

}
