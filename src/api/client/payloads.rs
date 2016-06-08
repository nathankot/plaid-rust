use api::types::*;
use api::client::Client;

use rustc_serialize::{Decodable, Decoder, Encodable, Encoder};
use rustc_serialize::json;

/// Represents a request for creating a new user
pub struct Authenticate<'a> {
    pub client: Client<'a>,
    pub username: Username,
    pub password: Password,
    pub institution: Institution,
}

impl<'a> Encodable for Authenticate<'a> {

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
