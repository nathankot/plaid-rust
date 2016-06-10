//! Strucutres to encapsulate an email as returned by the Plaid API

use rustc_serialize::{ Decoder, Decodable };

#[derive(Debug)]
/// A user's email, including meta data returned by Plaid.
pub struct Email {
    /// Whether or not the user has chosen this as their primary email.
    pub primary: bool,
    /// The designated type for this email (e.g personal, home).
    pub email_type: String,
    /// The actual email address.
    pub email: String
}

impl Decodable for Email {

    fn decode<D: Decoder>(d: &mut D) -> Result<Email, D::Error> {
        d.read_struct("root", 3, |d| {
            Ok(Email {
                primary: try!(d.read_struct_field("primary", 0, |d| d.read_bool())),
                email_type: try!(d.read_struct_field("type", 1, |d| d.read_str())),
                email: try!(d.read_struct_field("data", 2, |d| d.read_str()))
            })
        })
    }

}

#[cfg(test)]
mod tests {

    use api::data::*;
    use rustc_serialize::json;

    #[test]
    fn decode_email_works() {
        let x: Email = json::decode(r##"
            { "primary": true,
              "type": "personal",
              "data": "kelly.walters30@example.com" }
        "##).unwrap();

        assert_eq!(true, x.primary);
        assert_eq!("personal".to_string(), x.email_type);
        assert_eq!("kelly.walters30@example.com".to_string(), x.email);
    }

}
