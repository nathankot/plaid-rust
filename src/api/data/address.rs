//! Stores types that represent addresses

use rustc_serialize::{ Decodable, Decoder };

#[derive(Debug)]
/// A user's address, as returned by Plaid.
pub struct Address {
    /// The address zip code part.
    pub zip: Option<String>,
    /// The address state part.
    pub state: Option<String>,
    /// The address city part.
    pub city: Option<String>,
    /// The address street part.
    pub street: Option<String>,
    /// Longitude.
    pub longitude: Option<f64>,
    /// Latitude.
    pub latitude: Option<f64>
}

impl Decodable for Address {

    fn decode<D: Decoder>(d: &mut D) -> Result<Address, D::Error> {
        d.read_struct("address", 6, |d| {
            let (lat, lon) = try!(d.read_struct_field("coordinates", 0, |d| {
                d.read_option(|d, exists| {
                    if !exists { return Ok((None, None)) }
                    d.read_struct("coordinates", 2, |d| {
                        let lat: Option<f64> = d.read_struct_field("lat", 0, |d| Decodable::decode(d)).ok();
                        let lon: Option<f64> = d.read_struct_field("lon", 1, |d| Decodable::decode(d)).ok();
                        Ok((lat, lon))
                    })
                })
            }));

            let address = try!(d.read_struct_field("address", 2, |d| {
                d.read_option(|d, exists|
                              if exists { Decodable::decode(d) }
                              else { Ok(None) })
            }));

            let street = try!(d.read_struct_field("street", 1, |d| {
                d.read_option(|d, exists|
                              if exists { Decodable::decode(d) }
                              else { Ok(None) })
            }));

            Ok(Address {
                zip: try!(d.read_struct_field("zip", 3, |d| Decodable::decode(d))),
                state: try!(d.read_struct_field("state", 4, |d| Decodable::decode(d))),
                city: try!(d.read_struct_field("city", 5, |d| Decodable::decode(d))),
                street: street.or(address),
                latitude: lat,
                longitude: lon
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
            { "zip": "94114",
              "state": "CA",
              "city": "San Francisco",
              "street": "3819 Greenhaven Ln",
              "coordinates": {
                  "lat": 40.74,
                  "lon": -74.00
              }
            }
        "##).unwrap();

        assert_eq!(Some("94114".to_string()), x.zip);
        assert_eq!(Some("CA".to_string()), x.state);
        assert_eq!(Some("San Francisco".to_string()), x.city);
        assert_eq!(Some("3819 Greenhaven Ln".to_string()), x.street);
        assert_eq!(Some(40.74 as f64), x.latitude);
        assert_eq!(Some(-74.00 as f64), x.longitude);
    }

    #[test]
    fn decode_address_with_different_key_for_street_works() {
        let x: Address = json::decode(r##"
            { "zip": "94114",
              "state": "CA",
              "city": "San Francisco",
              "address": "3819 Greenhaven Ln",
              "coordinates": {
                  "lat": 40.74,
                  "lon": -74.00
              }
            }
        "##).unwrap();

        assert_eq!(Some("94114".to_string()), x.zip);
        assert_eq!(Some("CA".to_string()), x.state);
        assert_eq!(Some("San Francisco".to_string()), x.city);
        assert_eq!(Some("3819 Greenhaven Ln".to_string()), x.street);
        assert_eq!(Some(40.74 as f64), x.latitude);
        assert_eq!(Some(-74.00 as f64), x.longitude);
    }
}
