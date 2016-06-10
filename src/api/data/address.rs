//! Stores types that represent addresses

use rustc_serialize::{ Decodable, Decoder };

#[derive(Debug)]
/// A user's address, as returned by Plaid.
pub struct Address {
    /// Whether or not the user has chosen this as their primary address
    pub primary: bool,
    /// The address zip code part.
    pub zip: String,
    /// The address state part.
    pub state: String,
    /// The address city part.
    pub city: String,
    /// The address street part.
    pub street: String
}

impl Decodable for Address {

    fn decode<D: Decoder>(d: &mut D) -> Result<Address, D::Error> {
        d.read_struct("root", 2, |d| {
            let primary = try!(d.read_struct_field("primary", 0, |d| d.read_bool()));
            d.read_struct_field("data", 1, |d| {
                d.read_struct("address", 4, |d| {
                    Ok(Address {
                        primary: primary,
                        zip: try!(d.read_struct_field("zip", 0, |d| d.read_str())),
                        state: try!(d.read_struct_field("state", 0, |d| d.read_str())),
                        city: try!(d.read_struct_field("city", 0, |d| d.read_str())),
                        street: try!(d.read_struct_field("street", 0, |d| d.read_str()))
                    })
                })
            })
        })
    }

}

#[cfg(test)]
mod tests {

    use api::data::address::Address;
    use rustc_serialize::json;

    #[test]
    fn decode_address_works() {
        let x: Address = json::decode(r##"
            { "primary": true,
               "data": {
                 "zip": "94114",
                 "state": "CA",
                 "city": "San Francisco",
                 "street": "3819 Greenhaven Ln"
               }
            }
        "##).unwrap();

        assert_eq!(true, x.primary);
        assert_eq!("94114".to_string(), x.zip);
        assert_eq!("CA".to_string(), x.state);
        assert_eq!("San Francisco".to_string(), x.city);
        assert_eq!("3819 Greenhaven Ln".to_string(), x.street);
    }

}
