//! This module contains data structures and implementations
//! related to multi-factor-authentication.

use rustc_serialize::{Decodable, Decoder, Encodable, Encoder};

/// Represents one of the different types of multi-factor-authentication
/// challenges Plaid supports.
///
/// Todo: support all mfa challenges
#[derive(Debug, Eq, PartialEq)]
pub enum Challenge {
    /// A token-based authorization, this token will be sent to one of
    /// the user's registered devices.
    Code,
    /// A list of possible challenge devices, in which the user should
    /// choose one and then pass along the selection using `api::client::payload::AuthenticateOptions`.
    /// It is in the form of `(device_type, device_mask)`.
    DeviceList(Vec<(Device, String)>)
}

/// Represents a response to a previously given MFA challenge.
#[derive(Debug, Eq, PartialEq)]
pub enum Response {
    /// A response to a code challenge, providing the code
    /// that was sent to the user's device.
    Code(String)
}

impl Encodable for Response {

    fn encode<E: Encoder>(&self, e: &mut E) -> Result<(), E::Error> {
        match *self {
            Response::Code(ref str) => e.emit_str(str)
        }
    }

}

/// An internal `newtype` over `(Device, String)` to help with de-serialization
struct DeviceAndMask(Device, String);

impl Decodable for DeviceAndMask {

    fn decode<D: Decoder>(decoder: &mut D) -> Result<DeviceAndMask, D::Error> {
        decoder.read_struct("root", 2, |d| {
            let mask = try!(d.read_struct_field("mask", 0, |d| Decodable::decode(d)));
            let device = try!(d.read_struct_field("type", 1, |d| Decodable::decode(d)));
            Ok(DeviceAndMask(device, mask))
        })
    }

}

impl Decodable for Challenge {

    fn decode<D: Decoder>(d: &mut D) -> Result<Challenge, D::Error> {
        d.read_struct("root", 2, |d| {
            let t: String = try!(d.read_struct_field("type", 0, |d| Decodable::decode(d)));
            match t.as_ref() {
                "device" => Ok(Challenge::Code),
                "list" => {
                    let list: Vec<DeviceAndMask> = try!(d.read_struct_field("mfa", 0, |d| Decodable::decode(d)));
                    let list = list.iter().map(|&DeviceAndMask(ref device, ref mask)| { (*device, mask.clone()) }).collect();
                    Ok(Challenge::DeviceList(list))
                },
                p => Err(d.error(format!("Un-supported MFA preference: {}", p).as_ref()))
            }
        })
    }

}

/// Represents a device that can be used for multifactor authentication
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Device {
    /// Code sent to the user's email
    Email,
    /// Code sent to the user's phone number via sms
    Phone,
    /// Verify a credit card number
    Card
}

impl Decodable for Device {

    fn decode<D: Decoder>(decoder: &mut D) -> Result<Device, D::Error> {
        let s = try!(decoder.read_str());
        match s.as_ref() {
            "email" => Ok(Device::Email),
            "phone" => Ok(Device::Phone),
            "card" => Ok(Device::Card),
            _ => Err(decoder.error("Unknown mfa device type"))
        }
    }

}

impl Encodable for Device {

    fn encode<E: Encoder>(&self, e: &mut E) -> Result<(), E::Error> {
        match *self {
            Device::Email => e.emit_str("email"),
            Device::Phone => e.emit_str("phone"),
            Device::Card => e.emit_str("card")
        }
    }

}

#[cfg(test)]
mod tests {

    use api::mfa;
    use api::mfa::{Device, Challenge};
    use rustc_serialize::json;

    #[test]
    fn test_decoding_code_challenges() {
        let r: mfa::Challenge = json::decode(r##"
            { "type": "device",
              "mfa": { "message": "Code sent to xxx-xxx-5309" },
              "access_token": "xxxxx" }
        "##).unwrap();
        assert_eq!(format!("{:?}", r), "Code");
    }

    #[test]
    fn test_decoding_device_list_challenges() {
        let r: mfa::Challenge = json::decode(r##"
            {   "type": "list",
                "mfa": [
                    {"mask":"t..t@plaid.com", "type":"email"},
                    {"mask":"xxx-xxx-5309", "type":"phone"},
                    {"mask":"SafePass Card", "type":"card"}
                ],
                "access_token": "xxxxx" }
        "##).unwrap();
        assert_eq!(format!("{:?}", r), format!("{:?}", Challenge::DeviceList(vec![
            (Device::Email, "t..t@plaid.com".to_string()),
            (Device::Phone, "xxx-xxx-5309".to_string()),
            (Device::Card, "SafePass Card".to_string())
        ])));
    }

    #[test]
    fn test_decoding_device() {
        let r: mfa::Device = json::decode(r##""email""##).unwrap();
        assert_eq!(format!("{:?}", r), "Email");
        let r: mfa::Device = json::decode(r##""phone""##).unwrap();
        assert_eq!(format!("{:?}", r), "Phone");
        let r: mfa::Device = json::decode(r##""card""##).unwrap();
        assert_eq!(format!("{:?}", r), "Card");
    }

}
