//! User module

use api::product::*;
use api::client::{ Response };
use api::types::*;

use rustc_serialize::{Decodable, Decoder };

/// # User
/// Represents an authorized user for a given product.
#[derive(Debug)]
pub struct User<P: Product> {
    /// The result of the previous api request that returned this `Struct`
    pub status: Response<P>,
    /// The access token for this user
    pub access_token: AccessToken
}

impl<'a, P: Product> Decodable for User<P> {

    fn decode<D: Decoder>(decoder: &mut D) -> Result<User<P>, D::Error> {
        decoder.read_struct("root", 3, |decoder| {
            let access_token = try!(decoder.read_struct_field("access_token", 0, |d| Decodable::decode(d)));
            Ok(User {
                access_token: access_token,
                status: Response::Unknown
            })
        })
    }

}
