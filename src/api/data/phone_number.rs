//! Types to encapsulate a phone number, as represented by
//! the Plaid API.

use rustc_serialize::{ Decoder, Decodable };

#[derive(Debug)]
/// A user's phone number, as returned by Plaid.
pub struct PhoneNumber {
    /// Whether or not the user has chosen this as their primary phone number.
    pub primary: bool,
    /// The type of the phone number (e.g personal, home).
    pub phone_number_type: String,
    /// The actual phone number.
    pub phone_number: String
}

impl Decodable for PhoneNumber {

    fn decode<D: Decoder>(d: &mut D) -> Result<PhoneNumber, D::Error> {
        d.read_struct("root", 3, |d| {
            Ok(PhoneNumber {
                primary: try!(d.read_struct_field("primary", 0, |d| d.read_bool())),
                phone_number_type: try!(d.read_struct_field("type", 1, |d| d.read_str())),
                phone_number: try!(d.read_struct_field("data", 2, |d| d.read_str()))
            })
        })
    }

}

#[cfg(test)]
mod tests {

    use api::data::*;
    use rustc_serialize::json;

    #[test]
    fn decode_phone_number_works() {
        let x: PhoneNumber = json::decode(r##"
            {
                "primary": true,
                "type": "home",
                "data": "4673956022"
            }
        "##).unwrap();

        assert_eq!(true, x.primary);
        assert_eq!("home".to_string(), x.phone_number_type);
        assert_eq!("4673956022".to_string(), x.phone_number);
    }

}
